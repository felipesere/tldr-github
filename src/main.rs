#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::io::Read;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use async_std::prelude::*;
use async_std::stream;
use async_std::task;
use serde::Serialize;
use tide::middleware::RequestLogger;
use tide::{Request, Response};
use tide_naive_static_files::StaticFilesEndpoint;
use tracing::{event, instrument, span, Level};

use config::Config;
use db::Db;
use domain::api::{AddNewRepo, AddTrackedItemsForRepo, Repo};
use domain::ClientForRepositories;
use github::GithubClient;
use percent_encoding::percent_decode_str;

mod config;
mod db;
mod domain;
mod github;

embed_migrations!("./migrations");

struct State {
    db: Arc<dyn Db>,
    github: Arc<dyn ClientForRepositories>,
}

impl State {
    fn db(&self) -> Arc<dyn Db> {
        self.db.clone()
    }

    fn client(&self) -> Arc<dyn ClientForRepositories> {
        self.github.clone()
    }
}

fn from_url(val: String) -> String {
    percent_decode_str(&val)
        .decode_utf8()
        .expect("expected to decode value from URL")
        .to_string()
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut f = std::fs::File::open("./config.json")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let config: Config =
        serde_json::from_str(&contents).with_context(|| "Unable to read config")?;

    let db_access = config.database.get().unwrap();

    // let db_access = Arc::new(crate::db::in_memory::new());
    let github_access = Arc::new(GithubClient::new(config.github.token.clone()));

    let state = State {
        db: db_access.clone(),
        github: github_access.clone(),
    };

    let mut app = tide::with_state(state);
    app.middleware(RequestLogger::new());
    app.at("/").get(tide::redirect("/files/index.html"));
    app.at("/files").strip_prefix().get(StaticFilesEndpoint {
        root: "./tldr-github-svelte/public".into(),
    });
    app.at("/api").nest(|r| {
        r.at("/repos").get(|req: Request<State>| async move {
            let span = span!(Level::INFO, "GET /repos");
            let guard = span.enter();
            let db = req.state().db();

            let res = ApiResult::from(get_all_repos(db).with_context(|| "failed to get all repos"));
            drop(guard);
            res
        });
        r.at("/repos").post(|mut req: Request<State>| async move {
            let span = span!(Level::INFO, "POST /repos");
            let guard = span.enter();

            let client = req.state().client();
            let db = req.state().db();
            let add_repo: AddNewRepo = req.body_json().await.unwrap();

            let res = ApiResult::empty(
                domain::add_new_repo(db, client, add_repo.name)
                    .with_context(|| "failed to add repo"),
            );
            drop(guard);
            res
        });
        r.at("/repos/:name/tracked")
            .post(|mut req: Request<State>| async move {
                let name = from_url(req.param("name").unwrap());
                let client = req.state().client();
                let db = req.state().db();
                let body: AddTrackedItemsForRepo = req.body_json().await.unwrap();

                let maybe_repo = db.find_repo(&name);

                if maybe_repo.is_none() {
                    return ApiResult::not_found();
                }

                let repo = maybe_repo.unwrap();

                ApiResult::empty(
                    domain::add_items_to_track(db, client, repo, body.items)
                        .await
                        .with_context(|| "failed to add items to track"),
                )
            });
        r.at("/repos/:name/proxy")
            .get(|req: Request<State>| async move {
                let name = from_url(req.param("name").unwrap());
                let client = req.state().client();
                let db = req.state().db();

                let maybe_repo = db.find_repo(&name);

                if maybe_repo.is_none() {
                    return ApiResult::not_found();
                }

                let repo = maybe_repo.unwrap();

                ApiResult::from(
                    domain::retrieve_live_items(client, repo)
                        .await
                        .with_context(|| "failed to add items to track"),
                )
            });
        r.at("/repos/:name")
            .delete(|req: Request<State>| async move {
                let db = req.state().db();
                let name = from_url(req.param("name").unwrap());

                let maybe_repo = db.find_repo(&name);

                if maybe_repo.is_none() {
                    return ApiResult::not_found();
                }

                let repo = maybe_repo.unwrap();
                ApiResult::empty(db.delete(repo).with_context(|| "failed to delete"))
            });
    });
    // this doesn't work because every GET request gets redirected here

    if config.updater.run {
        let db = db_access.clone();
        let (sender, receiver) = async_std::sync::channel(100);
        task::spawn(async move {
            let mut interval = stream::interval(Duration::from_secs(30));
            while let Some(_) = interval.next().await {
                let all_repos = db.all().unwrap();

                for repo in all_repos {
                    for item in repo.items() {
                        sender.send((repo.stored(), item)).await;
                    }
                }
            }
        });

        let github = github_access.clone();
        domain::updater::start(domain::updater::Config {
            channel: receiver,
            client: github,
            db: db_access.clone(),
        });
    }

    task::block_on(async move { app.listen(config.server.address()).await })
        .with_context(|| "failed launch the server")
}

impl<T: Send + Sized + Serialize> tide::IntoResponse for ApiResult<T> {
    fn into_response(self) -> Response {
        use ApiResult::*;

        match self {
            Empty => Response::new(200),
            Success(val) => Response::new(200).body_json(&val).unwrap(),
            Failure(err) => Response::new(err.status)
                .body_json(&ErrorJson {
                    error: format!("{:#}", err.error),
                })
                .unwrap(),
        }
    }
}

impl<T> std::convert::From<anyhow::Result<T>> for ApiResult<T> {
    fn from(res: anyhow::Result<T>) -> ApiResult<T> {
        use ApiResult::*;

        match res {
            Ok(val) => Success(val),
            Err(e) => Failure(ApiError {
                status: 500,
                error: e,
            }),
        }
    }
}

enum ApiResult<T> {
    Empty,
    Success(T),
    Failure(ApiError),
}

impl<T> ApiResult<T> {
    fn not_found() -> ApiResult<T> {
        ApiResult::Failure(ApiError {
            status: 404,
            error: anyhow::anyhow!("Not found"),
        })
    }

    fn empty(result: anyhow::Result<T>) -> ApiResult<()> {
        use ApiResult::*;

        match result {
            Ok(_) => Empty,
            Err(e) => Failure(ApiError {
                status: 500,
                error: e,
            }),
        }
    }
}

struct ApiError {
    status: u16,
    error: anyhow::Error,
}

#[derive(Serialize)]
struct ErrorJson {
    error: String,
}

#[instrument(skip(db))]
fn get_all_repos(db: Arc<dyn Db>) -> anyhow::Result<Vec<domain::api::Repo>> {
    let repos = db.all()?;
    let mut result = Vec::new();
    for repo in repos {
        result.push(Repo::from(repo))
    }

    event!(Level::INFO, "Got {} repors to return", result.len());
    anyhow::Result::Ok(result)
}

pub trait BetterOption<T> {
    fn possibly(self, message: &'static str) -> anyhow::Result<T>;
}

impl<T> BetterOption<T> for Option<T> {
    fn possibly(self, message: &'static str) -> anyhow::Result<T> {
        match self {
            None => anyhow::Result::Err(anyhow::anyhow!(message)),
            Some(val) => anyhow::Result::Ok(val),
        }
    }
}

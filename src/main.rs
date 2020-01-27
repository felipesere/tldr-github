#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use anyhow::Context;
use async_std::task;
use serde::Serialize;
use tide::middleware::RequestLogger;
use tide::{Request, Response};
use tide_naive_static_files::StaticFilesEndpoint;
use tracing::{event, span, Level, instrument};
use tracing_subscriber;

use config::Config;
use domain::api::{AddNewRepo, AddTrackedItemsForRepo, Repo};
use domain::ClientForRepositories;
use github::GithubClient;

use db::{Db, SqliteDB};

mod config;
mod db;
mod domain;
mod filter;
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

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut f = std::fs::File::open("./config.json")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let config: Config =
        serde_json::from_str(&contents).with_context(|| "Unable to read config")?;

    let pool = config
        .database
        .setup()
        .with_context(|| "failed to setup DB")?;

    let ui = config.ui.clone();

    let pool = Arc::new(pool);

    let state = State {
        db: Arc::new(SqliteDB { conn: pool }),
        github: Arc::new(GithubClient::new(config.github.token.clone())),
    };

    let mut app = tide::with_state(state);
    app.middleware(RequestLogger::new());
    app.at("/").get(tide::redirect(ui.entry()));
    app.at(&ui.hosted_on)
        .strip_prefix()
        .get(StaticFilesEndpoint {
            root: ui.local_files.clone().into(),
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
                domain::add_new_repo(db, client, add_repo.name) .with_context(|| "failed to add repo"),
            );
            drop(guard);
            res
        });
        r.at("/repos/:id/tracked")
            .post(|mut req: Request<State>| async move {
                let id: i32 = req.param("id").unwrap();
                let client = req.state().client();
                let db = req.state().db();
                let body: AddTrackedItemsForRepo = req.body_json().await.unwrap();

                ApiResult::empty(
                    domain::add_items_to_track(db, client, id, body.items)
                        .await
                        .with_context(|| "failed to add items to track"),
                )
            });
        r.at("/repos/:id/proxy")
            .get(|req: Request<State>| async move {
                let id: i32 = req.param("id").unwrap();
                let client = req.state().client();
                let db = req.state().db();

                ApiResult::from(
                    domain::retrieve_live_items(db, client, id)
                        .await
                        .with_context(|| "failed to add items to track"),
                )
            });
        r.at("/repos/:id").delete(|req: Request<State>| async move {
            let db = req.state().db();
            let id = req.param::<i32>("id").unwrap();

            ApiResult::empty(db.delete(id).with_context(|| "failed to delete"))
        });
    });

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

#[instrument]
fn get_all_repos(db: Arc<dyn Db>) -> anyhow::Result<Vec<domain::api::Repo>> {

    let repos = db.all()?;
    let mut result = Vec::new();
    for repo in repos {
        result.push(Repo::from(repo))
    };

    event!(Level::INFO, "Got {} repors to return", result.len());
    anyhow::Result::Ok(result)
}

pub trait BetterOption<T> {
    fn possibly(self, messge: &'static str) -> anyhow::Result<T>;
}

impl<T> BetterOption<T> for Option<T> {
    fn possibly(self, message: &'static str) -> anyhow::Result<T> {
        match self {
            None => anyhow::Result::Err(anyhow::anyhow!(message)),
            Some(val) => anyhow::Result::Ok(val),
        }
    }
}

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
use simplelog::{CombinedLogger, LevelFilter, SharedLogger, TermLogger, TerminalMode, WriteLogger};
use tide::middleware::RequestLogger;
use tide::{Request, Response};
use tide_naive_static_files::StaticFilesEndpoint;

use config::Config;
use domain::api::Repo;
use domain::{ClientForRepositories, RepoName};
use github::GithubClient;

use db::{Db, SqliteDB};

mod config;
mod db;
mod domain;
mod filter;
mod github;

embed_migrations!("./migrations");

#[derive(serde::Deserialize, Debug)]
pub struct AddNewRepo {
    name: String,
}

struct State {
    db: Arc<dyn Db + Send + Sync>,
    github: Arc<dyn ClientForRepositories + Send + Sync>,
}

impl State {
    fn db(&self) -> Arc<dyn Db + Send + Sync> {
        self.db.clone()
    }

    fn client(&self) -> Arc<dyn ClientForRepositories + Send + Sync> {
        self.github.clone()
    }
}

pub fn terminal() -> Box<dyn SharedLogger> {
    TermLogger::new(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap()
}

pub fn file(name: &'static str) -> Box<dyn SharedLogger> {
    WriteLogger::new(
        LevelFilter::Info,
        simplelog::Config::default(),
        File::create(name).unwrap(),
    )
}

fn main() -> anyhow::Result<()> {
    let mut f = std::fs::File::open("./config.json")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let config: Config =
        serde_json::from_str(&contents).with_context(|| "Unable to read config")?;

    CombinedLogger::init(vec![terminal(), file("tldr-github.log")])
        .with_context(|| "failed to initialize the logging system")?;

    let pool = config
        .database
        .setup()
        .with_context(|| "failed to setup DB")?;

    let ui = config.ui.clone();

    let pool = Arc::new(pool);

    let state = State {
        db: Arc::new(SqliteDB { conn: pool.clone() }),
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
        r.at("/repos").get(|req: Request<State>| {
            async move {
                let db = req.state().db();

                ApiResult::from(get_all_repos(db).with_context(|| "failed to get all repos"))
            }
        });
        r.at("/repos").post(|mut req: Request<State>| {
            async move {
                let client = req.state().client();
                let db = req.state().db();
                let add_repo: AddNewRepo = req.body_json().await.unwrap();

                let name = RepoName::from(add_repo.name).unwrap();

                ApiResult::empty(
                    domain::add_new_repo(db, client, name).with_context(|| "failed to add repo"),
                )
            }
        });
        r.at("/repos/:id").delete(|req: Request<State>| {
            async move {
                let db = req.state().db();
                let id = req.param::<i32>("id").unwrap();

                ApiResult::empty(db.delete(id).with_context(|| "failed to delete"))
            }
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

fn get_all_repos(db: Arc<dyn Db>) -> anyhow::Result<Vec<domain::api::Repo>> {
    let repos = db.all()?;
    let mut result = Vec::new();
    for repo in repos {
        result.push(Repo::from(repo))
    }
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

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Context;
use async_std::task;
use serde::Serialize;
use simplelog::CombinedLogger;
use tide::{Request, Response};
use tide_naive_static_files::{serve_static_files, StaticRootDir};

use config::Config;
use domain::RepoName;
use github::GithubClient;
use middleware::logger;

use crate::db::{NewRepoEvent, RepoEvents};

mod config;
mod db;
mod domain;
mod github;
mod middleware;

embed_migrations!("./migrations");

#[derive(serde::Deserialize, Debug)]
pub struct AddNewRepo {
    name: String,
}

struct State {
    pool: Arc<db::SqlitePool>,
    static_root_dir: PathBuf,
    github: Arc<github::GithubClient>,
}

impl State {
    fn conn(&self) -> db::Conn {
        self.pool.get().unwrap()
    }

    fn client(&self) -> Arc<GithubClient> {
        self.github.clone()
    }
}

impl StaticRootDir for State {
    fn root_dir(&self) -> &Path {
        &self.static_root_dir
    }
}

fn main() -> anyhow::Result<()> {
    let mut file = std::fs::File::open("./config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config =
        serde_json::from_str(&contents).with_context(|| "Unable to read config")?;

    CombinedLogger::init(vec![logger::terminal(), logger::file("tldr-github.log")])
        .with_context(|| "failed to initialize the logging system")?;

    let pool = config
        .database
        .setup()
        .with_context(|| "failed to setup DB")?;

    let ui = config.ui.clone();

    let state = State {
        pool: Arc::new(pool),
        static_root_dir: ui.local_files.clone().into(),
        github: Arc::new(GithubClient::new(config.github.token.clone())),
    };

    let mut app = tide::with_state(state);
    app.middleware(logger::RequestLogger::new());
    app.at("/").get(tide::redirect(ui.entry()));
    app.at(&ui.hosted())
        .get(|req: Request<State>| async { serve_static_files(req).await.unwrap() });
    app.at("/api").nest(|r| {
        r.at("/repos").get(|req: Request<State>| {
            async move {
                let conn = req.state().conn();

                ApiResult::from(get_all_repos(&conn))
            }
        });
        r.at("/repos").post(|mut req: Request<State>| {
            async move {
                let client = req.state().client();
                let conn = req.state().conn();
                let add_repo: AddNewRepo = req.body_json().await.unwrap();

                ApiResult::empty(add_new_repo(&conn, &client, add_repo))
            }
        });
        r.at("/repos/:id").delete(|req: Request<State>| {
            async move {
                let conn = req.state().conn();
                let id = req.param::<i32>("id").unwrap();

                ApiResult::empty(db::delete(&conn, id).with_context(|| "failed to delete"))
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

impl ApiResult<()> {
    fn empty(result: anyhow::Result<()>) -> ApiResult<()> {
        use ApiResult::*;

        match result {
            Ok(()) => Empty,
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

fn add_new_repo(
    conn: &db::Conn,
    client: &GithubClient,
    repo_to_add: AddNewRepo,
) -> anyhow::Result<()> {
    let name = RepoName::from(repo_to_add.name)?;
    let pulls = client.pull_requests(&name).unwrap_or(Vec::new());
    let issues = client.issues(&name).unwrap_or(Vec::new());
    let last_commit = client.last_commit(&name);

    let repo = db::insert_new_repo(&conn, &name.to_string())?;
    db::insert_prs(&conn, &repo, pulls)?;
    db::insert_issues(&conn, &repo, issues)?;

    if let Ok(commit) = last_commit {
        let r = db::insert_new_repo_activity(
            conn,
            &repo,
            NewRepoEvent {
                event: RepoEvents::LatestCommitOnMaster(commit),
            },
        );

        if let Err(e) = r {
            log::error!("failed to insert new activity: {}", e)
        }
    }

    Ok(())
}

fn get_all_repos(conn: &db::Conn) -> anyhow::Result<Vec<domain::api::Repo>> {
    let repos = db::all_repos(&conn).unwrap();
    let mut result = Vec::new();
    for repo in repos {
        let pulls: Vec<domain::api::Item> = db::find_prs_for_repo(&conn, repo.id)
            .unwrap()
            .into_iter()
            .map(|pr| domain::api::Item {
                by: pr.by,
                title: pr.title,
                link: pr.link,
            })
            .collect();

        let issues: Vec<domain::api::Item> = db::find_issues_for_repo(&conn, repo.id)
            .unwrap()
            .into_iter()
            .map(|pr| domain::api::Item {
                by: pr.by,
                title: pr.title,
                link: pr.link,
            })
            .collect();

        let repo_event = db::find_last_activity_for_repo(&conn, repo.id);

        let mut last_commit = None;
        if let Some(existing_event) = repo_event {
            match existing_event.event {
                db::RepoEvents::LatestCommitOnMaster(c) => last_commit = Some(c),
            }
        }

        let r = domain::api::Repo {
            id: repo.id,
            title: repo.title,
            last_commit: last_commit.map(|c| domain::api::Commit::from(c)),
            activity: domain::api::Activity {
                master: domain::api::CommitsOnMaster { commits: 0 },
                prs: pulls,
                issues,
            },
        };

        result.push(r)
    }
    anyhow::Result::Ok(result)
}

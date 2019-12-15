#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

use std::io::Read;
use std::sync::Arc;

use anyhow::Context;
use async_std::task;
use github::graphql::GithubClient;
use middleware::logger;
use simplelog::CombinedLogger;
use std::path::{Path, PathBuf};
use tide::{Request, Response};
use domain::{RepoName};
use config::Config;
use tide_naive_static_files::{serve_static_files, StaticRootDir};
use serde::Serialize;

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
    github: Arc<github::graphql::GithubClient>,
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
                let c = req.state().conn();
                let client = req.state().client();
                let repos = db::all_repos(c).unwrap();

                let mut result = Vec::new();
                for repo in repos {
                    let name = match domain::RepoName::from(repo.title.clone()) {
                        Ok(n) => n,
                        Err(err) => {
                            log::error!("failure: {}", err);
                            continue;
                        }
                    };
                    let pulls = client.pull_requests(name.clone()).unwrap_or(Vec::new());
                    let issues = client.issues(name.clone()).unwrap_or(Vec::new());
                    let last_commit = client
                        .last_commit(name.clone())
                        .expect("there was no last commit");

                    let r = domain::Repo {
                        id: repo.id,
                        title: repo.title,
                        last_commit,
                        activity: domain::Activity {
                            master: domain::CommitsOnMaster { commits: 0 },
                            prs: pulls,
                            issues: issues,
                        },
                    };

                    result.push(r)
                }

                Response::new(200).body_json(&result).unwrap()
            }
        });
        r.at("/repos").post(|mut req: Request<State>| {
            async move {
                let add_repo: AddNewRepo = req.body_json().await.unwrap();
                let c = req.state().conn();

                match RepoName::from(add_repo.name) {
                    Ok(name) =>{
                        db::insert_new_repo(&c, &name.to_string()).unwrap();
                        Response::new(200)
                    },
                    Err(err) => {
                        log::error!("failure to add repo: {}", err);
                        error_response(404, err)
                    },
                }
            }
        });
        r.at("/repos/:id").delete(|req: Request<State>| {
            async move {
                let conn = req.state().conn();
                let id = req.param::<i32>("id").unwrap();

                match db::delete(&conn, id) {
                    Ok(_) => Response::new(200),
                    Err(err) => {
                        log::error!("failure to delete repo: {}", err);
                        error_response(400, err)
                    }
                }
            }
        });
    });

    task::block_on(async move { app.listen(config.server.address()).await })
        .with_context(|| "failed launch the server")
}

#[derive(Serialize)]
struct ErrorJson {
    error: String,
}

fn error_response(status: u16, error: anyhow::Error) -> Response {
    Response::new(status).body_json(&ErrorJson { error: format!("{:#}", error) }).unwrap()
}

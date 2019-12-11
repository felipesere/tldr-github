#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;


use std::sync::Arc;
use std::io::Read;

use async_std::task;
use anyhow::{Context};
use tide::{Request, Response};
use simplelog::CombinedLogger;
use middleware::logger;
use std::path::{Path, PathBuf};

use config::Config;

use tide_naive_static_files::{serve_static_files, StaticRootDir};

mod config;
mod db;
mod middleware;
mod github;
mod util;
mod domain;


embed_migrations!("./migrations");

#[derive(serde::Deserialize, Debug)]
pub struct AddNewRepo {
    name: String,
}

struct State {
    pool: Arc<db::SqlitePool>,
    static_root_dir: PathBuf,
}

impl State {
    fn conn(&self) -> db::Conn {
        self.pool.get().unwrap()
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

    let config: Config = serde_json::from_str(&contents).with_context(|| "Unable to read config")?;

    CombinedLogger::init(vec![logger::terminal(), logger::file("tldr-github.log") ]).with_context(|| "failed to initialize the logging system")?;


    let pool = config.database.setup().with_context(|| "failed to setup DB")?;

    let ui = config.ui.clone();

    let state = State {
        pool: Arc::new(pool),
        static_root_dir: ui.local_files.clone().into(),
    };

    let mut app = tide::with_state(state);
    app.middleware(logger::RequestLogger::new());
    app.at("/").get(tide::redirect(ui.entry()));
    app.at(&ui.hosted()).get(|req: Request<State>| async {
        serve_static_files(req).await.unwrap()
    });
    app.at("/api").nest(|r| {
        r.at("/repos").get(|req: Request<State>| async move {
            let c = req.state().conn();
            let repos = db::all_repos(c).unwrap();

            let client = github::graphql::GithubClient::new("<< token >>");

            let mut result = Vec::new();
            for repo in repos {
                let name = domain::RepoName::from(repo.title.clone()).unwrap();
                let pulls = client.pull_requests(name.clone()).unwrap_or(Vec::new());
                let issues = client.issues(name.clone()).unwrap_or(Vec::new());

                let r = domain::Repo {
                    title: repo.title,
                    last_commit: domain::sample::last_commit(),
                    activity: domain::Activity {
                        master: domain::CommitsOnMaster { commits: 0 },
                        prs: pulls,
                        issues: issues,
                    },
                };

                result.push(r)
            }

            Response::new(200).body_json(&result).unwrap()
        });
        r.at("/repos").post(|mut req: Request<State>| async move {
            let add_repo: AddNewRepo = req.body_json().await.unwrap();
            let c = req.state().conn();

            db::insert_new(&c, &add_repo.name).unwrap();
            Response::new(200)
        });
    });

    task::block_on(async move {
        app.listen(config.server.address()).await
    }).with_context(|| "failed launch the server")
}

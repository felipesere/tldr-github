#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;


use std::sync::Arc;
use std::io::Read;

use async_std::task;
use anyhow::{Context};
use tide::{Request, Response};
use simplelog::CombinedLogger;
use middleware::logger;
use middleware::static_files;

use config::Config;

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
}

impl State {
    fn conn(&self) -> db::Conn {
        self.pool.get().unwrap()
    }
}

fn main() -> anyhow::Result<()> {
    let mut file = std::fs::File::open("./config.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_json::from_str(&contents).with_context(|| "Unable to read config")?;

    CombinedLogger::init(vec![logger::terminal(), logger::file("tldr-github.log") ]).with_context(|| "failed to initialize the logging system")?;


    let pool = config.database.setup().with_context(|| "failed to setup DB")?;

    let state = State {
        pool: Arc::new(pool),
    };

    let ui = config.ui.clone();
    let mut app = tide::with_state(state);
    app.middleware(logger::RequestLogger::new());
    app.at("/").get(tide::redirect(ui.entry()));
    app.at(&ui.hosted()).get(static_files::in_dir(ui.local_files));
    app.at("/api").nest(|r| {
        r.at("/repos").get(|req: Request<State>| async move {
            let c = req.state().conn();
            let repos = db::all_repos(c).unwrap();
            Response::new(200).body_json(&repos).unwrap()
        });
        r.at("/repos").post(|mut req: Request<State>| async move {
            let add_repo: AddNewRepo = req.body_json().await.unwrap();
            let c = req.state().conn();

            db::insert_new(&c, &add_repo.name).unwrap();
            Response::new(200)
        });
        r.at("/repos/:id/issues").get(|req: Request<State>| async move {
            let id: Result<i32, std::num::ParseIntError> = req.param("id");
            let c = req.state().conn();
            let repo = match db::find_repo(&c, id.unwrap()) {
                Some(r) => r,
                None => return Response::new(404),
            };


            let uri = format!("https://api.github.com/repos/{}/issues", repo.title);
            let issues: Vec<github::Issue> = surf::get(uri).recv_json().await.unwrap();

            Response::new(200).body_json(&issues).unwrap()
        });
        r.at("/repos/:id/pulls").get(|req: Request<State>| async move {
            let id: Result<i32, std::num::ParseIntError> = req.param("id");
            let c = req.state().conn();
            let repo = match db::find_repo(&c, id.unwrap()) {
                Some(r) => r,
                None => return Response::new(404),
            };


            let uri = format!("https://api.github.com/repos/{}/pulls", repo.title);
            let issues: Vec<github::PullRequest> = surf::get(uri).recv_json().await.unwrap();

            Response::new(200).body_json(&issues).unwrap()
        });
    });

    task::block_on(async move {
        app.listen(config.server.address()).await
    }).with_context(|| "failed launch the server")
}

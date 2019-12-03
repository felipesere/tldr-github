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


embed_migrations!("./migrations");

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

    let mut app = tide::with_state(state);
    app.middleware(logger::RequestLogger::new());
    app.at("/").get(tide::redirect("/files/index.html"));
    app.at("/files/*filename").get(static_files::in_dir("./frontend"));
    app.at("/api").nest(|r| {
        r.at("/repos").get(|req: Request<State>| async move {
            let c = req.state().conn();
            let repos = db::all_repos(c).unwrap();
            Response::new(200).body_json(&repos).unwrap()
        });
        r.at("/repos/:id").get(|req: Request<State>| async move {
            let id: Result<i32, std::num::ParseIntError> = req.param("id");
            let c = req.state().conn();
            match db::find_repo(c, id.unwrap()) {
                Some(repo) => Response::new(200).body_json(&repo).unwrap(),
                None => Response::new(404),
            }
        });
        r.at("/github/*repo").get(|req: Request<State>| async move {
            let repo: String = req.param("repo").unwrap();
            let uri = format!("https://api.github.com/repos/{}", repo);
            log::info!("Reaching out to: {}", uri);
            let r = surf::get(uri).recv_string().await.unwrap();

            Response::new(200).body_string(r).set_header("Content-Type", "application/json")
        });
    });

    task::block_on(async move {
        app.listen(config.server.address()).await
    }).with_context(|| "failed launch the server")
}

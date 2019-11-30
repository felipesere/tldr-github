#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;


use std::sync::Arc;

use async_std::task;
use anyhow::{Context};
use tide::{Request, Response};
use simplelog::*;

mod static_files;
mod logger;
mod db;


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
    CombinedLogger::init(vec![logger::terminal(), logger::file("tldr-github.log") ]).with_context(|| "failed to initialize the logging system")?;

    let pool = db::establish_connection()?;
    embedded_migrations::run_with_output(&pool.get().unwrap(), &mut std::io::stdout());

    let state = State {
        pool: Arc::new(pool),
    };

    let files = crate::static_files::new::<State>();

    let mut app = tide::with_state(state);
    app.middleware(logger::RequestLogger::new());
    app.at("/").get(tide::redirect("/index.html"));
    app.at("/files").nest(files.router());
    app.at("/api").nest(|r| {
        r.at("/felipe").get(|mut req: Request<State>| async move {
            let c = req.state().conn();
            let repos = db::all_repos(c).unwrap();
            Response::new(200).body_json(&repos).unwrap()
        });
    });

    task::block_on(async move {
        app.listen("127.0.0.1:8080").await
    }).with_context(|| "failed launch the server")
}

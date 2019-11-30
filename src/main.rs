#[macro_use]
extern crate diesel_migrations;

use std::sync::Arc;

use async_std::task;
use anyhow::{Context, Result};
use tide::{Request, Response};
use simplelog::*;
use diesel::sqlite::SqliteConnection;
use diesel::connection::Connection;
use diesel::r2d2::{self, ConnectionManager};

mod static_files;
mod logger;


embed_migrations!("./migrations");

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

struct State {
    pool: Arc<Pool>,
}

impl State {
    fn conn(&self) -> impl Connection {
        self.pool.get().unwrap()
    }
}

pub fn establish_connection() -> Result<Pool> {
    let database_url = "repos.db";
    Pool::new(ConnectionManager::new(database_url)).with_context(|| format!("failed to access db: {}", database_url))
}

fn main() -> anyhow::Result<()> {
    CombinedLogger::init(vec![logger::terminal(), logger::file("tldr-github.log") ]).with_context(|| "failed to initialize the logging system")?;

    let pool = establish_connection()?;
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
            let _c = req.state().conn();
            "I borrowed a connection!"
        });
    });

    task::block_on(async move {
        app.listen("127.0.0.1:8080").await
    }).with_context(|| "failed launch the server")
}

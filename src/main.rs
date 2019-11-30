#[macro_use]
extern crate diesel_migrations;

use std::fs::File;
use std::sync::Arc;

use async_std::task;
use anyhow::{Context, Result};
use futures::future::BoxFuture;
use tide::{Middleware, Next, Request, Response};
use simplelog::*;
use diesel::sqlite::SqliteConnection;
use diesel::connection::Connection;
use diesel::r2d2::{self, ConnectionManager};

mod static_files;


embed_migrations!("./migrations");

#[derive(Debug, Clone, Default)]
pub struct RequestLogger;

impl RequestLogger {
    pub fn new() -> Self {
        Self::default()
    }

    async fn log_basic<'a, State: Send + Sync + 'static>(
        &'a self,
        ctx: Request<State>,
        next: Next<'a, State>,
    ) -> Response {
        let path = ctx.uri().path().to_owned();
        let method = ctx.method().as_str().to_owned();
        log::trace!("IN => {} {}", method, path);
        let start = std::time::Instant::now();
        let res = next.run(ctx).await;
        let status = res.status();
        log::info!(
            "{} {} {} {}ms",
            method,
            path,
            status.as_str(),
            start.elapsed().as_millis()
        );
        res
    }
}

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

struct State {
    pool: Arc<Pool>,
}

impl State {
    fn conn(&self) -> impl Connection {
        self.pool.get().unwrap()
    }
}

impl<State: Send + Sync + 'static> Middleware<State> for RequestLogger {
    fn handle<'a>(&'a self, ctx: Request<State>, next: Next<'a, State>) -> BoxFuture<'a, Response> {
        Box::pin(async move { self.log_basic(ctx, next).await })
    }
}

pub fn establish_connection() -> Pool {
    let database_url = "repos.db";
    Pool::new(ConnectionManager::new(database_url)).expect(&format!("Error connecting to {}", database_url))
}

fn main() -> anyhow::Result<()> {
    CombinedLogger::init(
        vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(LevelFilter::Info, Config::default(), File::create("tldr-github.log").unwrap()),
        ]
    ).unwrap();

    let pool = establish_connection();
    embedded_migrations::run_with_output(&pool.get().unwrap(), &mut std::io::stdout());

    let state = State {
        pool: Arc::new(pool),
    };

    let files = crate::static_files::new::<State>();

    let mut app = tide::with_state(state);
    app.middleware(RequestLogger{});
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

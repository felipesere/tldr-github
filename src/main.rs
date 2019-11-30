use std::result::Result;
use std::fs::File;

use async_std::task;
use futures::future::BoxFuture;
use tide::{Middleware, Next, Request, Response};
use simplelog::*;

mod static_files;


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

impl<State: Send + Sync + 'static> Middleware<State> for RequestLogger {
    fn handle<'a>(&'a self, ctx: Request<State>, next: Next<'a, State>) -> BoxFuture<'a, Response> {
        Box::pin(async move { self.log_basic(ctx, next).await })
    }
}

fn main() -> Result<(), std::io::Error> {
    CombinedLogger::init(
        vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(LevelFilter::Info, Config::default(), File::create("tldr-github.log").unwrap()),
        ]
    ).unwrap();


    let files = crate::static_files::new::<()>();

    let mut app = tide::new();
    app.middleware(RequestLogger{});
    app.at("/").get(tide::redirect("/index.html"));
    app.at("/files").nest(files.router());

    task::block_on(async move {
        app.listen("127.0.0.1:8080").await
    })
}

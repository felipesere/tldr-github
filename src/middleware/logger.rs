use futures::future::BoxFuture;
use tide::{Middleware, Next, Request, Response};
use std::fs::File;

use simplelog::*;

#[derive(Debug, Clone, Default)]
pub struct RequestLogger;

pub fn terminal() -> Box<dyn SharedLogger> {
    TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap()
}

pub fn file(name: &'static str) -> Box<dyn SharedLogger> {
    WriteLogger::new(LevelFilter::Info, Config::default(), File::create(name).unwrap())
}

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

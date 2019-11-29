use std::result::Result;
use std::fs::File;

use async_std::task;

use tide::{ Middleware, Next, Request, Response};
use futures::future::BoxFuture;

use simplelog::*;

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

    let mut app = tide::new();
    app.middleware(RequestLogger{});
    app.at("/").get(tide::redirect("/index.html"));

    // TODO:    turn this into a "static files" router that matches against local files
    //          exclude evil paths like "/.."
    //          https://github.com/SergioBenitez/Rocket/blob/da7e022f990e0b8e8201b0a359a43104686ff1a4/core/http/src/uri/segments.rs#L65
    app.at("/*filename").get(|req: Request<()>| async move {
        let filename: String = req.param("filename").unwrap();
        log::warn!("The filename was: {}", filename);

        match async_std::fs::read_to_string(&filename).await {
            Ok(content) => {
                let path = std::path::Path::new(&filename);
                match path.extension() {
                    Some(extension) => { 
                        let content_type = format!("text/{}", (*extension).to_str().unwrap());
                        Response::new(200).body_string(content).set_header("Content-Type", content_type)
                    },
                    None => Response::new(200).body_string(content),
                }
            },
            Err(_) => Response::new(404),
        }
    });
    task::block_on(async move {
        app.listen("127.0.0.1:8080").await;
    });
    Ok(())
}

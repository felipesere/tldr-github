use tide::{Endpoint, Request, Response};
use std::future::Future;
use async_std::sync::{Arc, Mutex};

// TODO:turn this into a "static files" router that matches against local files
//      exclude evil paths like "/.."
//      https://github.com/SergioBenitez/Rocket/blob/da7e022f990e0b8e8201b0a359a43104686ff1a4/core/http/src/uri/segments.rs#L65
pub struct StaticFilesV2 {
    pub root: String,
}

pub(crate) type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;

impl <STATE: Send + Sync + 'static> Endpoint<STATE> for StaticFilesV2 {
    type Fut = BoxFuture<'static, Response>;
    fn call(&self, req: Request<STATE>) -> Self::Fut {
        let protected_request = Arc::new(Mutex::new( (req, self.root.clone()) ));
        Box::pin(async move {
            let (request, root) = &*protected_request.lock().await;

            let filename: String = request.param("filename").unwrap();
            log::warn!("The filename was: {}", filename);

            let real_filename = &format!("{}/{}", root , filename);
            let path = std::path::Path::new(&real_filename);

            match std::fs::read_to_string(path) {
                Ok(content) => {
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
        })
    }
}

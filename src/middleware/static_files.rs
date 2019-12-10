use tide::{Endpoint, Request, Response};
use std::future::Future;
use async_std::sync::{Arc, Mutex};

// TODO More ideas:
//  * a macro that underneath uses include_str/include_bin to pump all files
//  into the binary AND sets up adequate routes for ultimate speed.
//
//  * similar variation as a bove, but reads all files during boot without bloating the binary

pub struct StaticFiles {
    pub root: String,
}

pub fn in_dir<S: Into<String>>(root: S) -> StaticFiles {
    StaticFiles {
        root: root.into(),
    }
}

impl StaticFiles {
    fn content_type(path: &std::path::Path) -> String {
        match path.extension() {
            Some(extension) => {
                format!("text/{}", (*extension).to_str().unwrap())
            },
            None => "text/plain".into()
        }
    }
}

pub(crate) type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 'a>>;


// TODO:turn this into a "static files" router that matches against local files
//      exclude evil paths like "/.."
//      https://github.com/SergioBenitez/Rocket/blob/da7e022f990e0b8e8201b0a359a43104686ff1a4/core/http/src/uri/segments.rs#L65
impl <STATE: Send + Sync + 'static> Endpoint<STATE> for StaticFiles {
    type Fut = BoxFuture<'static, Response>;
    fn call(&self, req: Request<STATE>) -> Self::Fut {
        // TODO: there must be a better way than this?
        let protected_request = Arc::new(Mutex::new( (req, self.root.clone()) ));
        Box::pin(async move {
            let (request, root) = &*protected_request.lock().await;

            let filename: String = request.param("filename").unwrap();
            log::warn!("The request file was: {}", filename);

            let real_filename = &format!("{}/{}", root , filename);
            log::warn!("The expanded file was: {}", real_filename);

            let path = std::path::Path::new(&real_filename);

            match std::fs::read_to_string(path) {
                Ok(content) => Response::new(200).body_string(content).set_header("Content-Type", StaticFiles::content_type(path)),
                Err(err) => Response::new(404).body_string(err.to_string()),
            }
        })
    }
}

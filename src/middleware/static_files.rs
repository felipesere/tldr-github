use tide::{ Request, Response};

// TODO:turn this into a "static files" router that matches against local files
//      exclude evil paths like "/.."
//      https://github.com/SergioBenitez/Rocket/blob/da7e022f990e0b8e8201b0a359a43104686ff1a4/core/http/src/uri/segments.rs#L65
pub struct StaticFiles<STATE: Send + Sync + 'static> {
    _marker: std::marker::PhantomData<STATE>,
}

pub fn new<STATE: Send + Sync + 'static>() -> StaticFiles<STATE> {
    return StaticFiles {
        _marker: std::marker::PhantomData,
    }
}

impl <STATE: Send + Sync + 'static> StaticFiles<STATE> {
    pub fn router(self) -> impl FnOnce(&mut tide::Route<'_, STATE>) {
        return |base: &mut tide::Route<STATE>| {
            base.at("/*filename").get(|req: Request<STATE>| async move {
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
        }
    }
}

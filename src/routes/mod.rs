use webapp::UriValue;
use webapp::status::StatusCode;
use route_recognizer::{Router, Params};
use modules::Modules;

mod files;
mod model;

pub use self::model::{model_handler, RouteModel, ModelHandlerWrapper, ModelRouteHandler};
pub use self::files::file_handler;

pub struct Routes {
    handlers: Router<HandlerEntry>
}

impl Routes {
    pub fn new() -> Self {
        Routes {
            handlers: Router::new()
        }
    }

    pub fn register<H: RouteHandler + 'static>(&mut self, route: &str, handler: H) {
        self.handlers.add(route, HandlerEntry {
            callback: Box::new(handler),
        });
    }

    pub fn handle(&self, modules: &Modules, route: &str) -> Result<Vec<u8>, StatusCode> {
        if let Ok(matc) = self.handlers.recognize(route) {
            let params = matc.params;
            let entry = matc.handler;

            let url = UrlParams {
                internal: params
            };

            Ok(entry.callback.handle(modules, url))
        } else {
            Err(StatusCode::NotFound)
        }
    }
}

struct HandlerEntry {
    callback: Box<RouteHandler>,
}

pub trait RouteHandler: Send + Sync {
    fn handle(&self, modules: &Modules, url: UrlParams) -> Vec<u8>;
}

impl<F: Fn(&Modules, UrlParams) -> Vec<u8> + Send + Sync> RouteHandler for F {
    fn handle(&self, modules: &Modules, url: UrlParams) -> Vec<u8> {
        self(modules, url)
    }
}

#[derive(Debug)]
pub struct UrlParams {
    internal: Params
}

impl UrlParams {
    pub fn get(&self, key: &str) -> Option<String> {
        let raw = try_opt!(self.internal.find(key));
        let val = UriValue::bless(raw);
        Some(val.unescape())
    }
}

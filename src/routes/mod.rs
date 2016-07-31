use webapp::UriValue;
use webapp::status::StatusCode;
use webapp::method::Method;
use route_recognizer::{Router, Params};
use modules::Modules;

mod files;
mod model;

pub use self::model::{model_handler, RouteModel, ModelHandlerWrapper, ModelRouteHandler};
pub use self::files::file_handler;

pub struct Routes {
    get_handlers: Router<Box<RouteHandler>>,
    post_handlers: Router<Box<RouteHandler>>,
}

impl Routes {
    pub fn new() -> Self {
        Routes {
            get_handlers: Router::new(),
            post_handlers: Router::new()
        }
    }

    pub fn get<H: RouteHandler + 'static>(&mut self, route: &str, handler: H) {
        self.register(Method::Get, route, handler);
    }

    pub fn post<H: RouteHandler + 'static>(&mut self, route: &str, handler: H) {
        self.register(Method::Post, route, handler);
    }

    pub fn register<H: RouteHandler + 'static>(&mut self, method: Method, route: &str, handler: H) {
        self.get_router_mut(method).add(route, Box::new(handler));
    }

    pub fn handle(&self, modules: &Modules, method: Method, route: &str) -> Result<Vec<u8>, StatusCode> {
        if let Ok(matc) = self.get_router(method).recognize(route) {
            let params = matc.params;
            let entry = matc.handler;

            let url = UrlParams {
                internal: params
            };

            Ok(entry.handle(modules, url))
        } else {
            Err(StatusCode::NotFound)
        }
    }

    fn get_router(&self, method: Method) -> &Router<Box<RouteHandler>> {
        match method {
            Method::Get => &self.get_handlers,
            Method::Post => &self.post_handlers,
            _ => unimplemented!(), // TODO: IMPORTANT FOR PRODUCTION Do not panic
        }
    }

    fn get_router_mut(&mut self, method: Method) -> &mut Router<Box<RouteHandler>> {
        match method {
            Method::Get => &mut self.get_handlers,
            Method::Post => &mut self.post_handlers,
            _ => unimplemented!(), // TODO: IMPORTANT FOR PRODUCTION Do not panic
        }
    }
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

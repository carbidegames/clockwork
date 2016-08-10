use route_recognizer::Router;
use webapp::status::StatusCode;
use webapp::method::Method;
use modules::Modules;

mod files;
mod model;
mod params;

pub use self::model::{model_handler, RouteModel, ModelHandlerWrapper, ModelRouteHandler};
pub use self::files::file_handler;
pub use self::params::{UriParams, BodyParams};

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

    pub fn handle(&self, modules: &Modules, method: Method, route: &str, body: Vec<u8>)
     -> Result<Vec<u8>, StatusCode> {
        if let Ok(matc) = self.get_router(method).recognize(route) {
            let params = matc.params;
            let entry = matc.handler;

            let url = params::url_params_from_route_recognizer(params);
            let body = params::body_params_from_data(body);

            Ok(entry.handle(modules, url, body))
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
    fn handle(&self, modules: &Modules, url: UriParams, body: BodyParams) -> Vec<u8>;
}

impl<F: Fn(&Modules, UriParams, BodyParams) -> Vec<u8> + Send + Sync> RouteHandler for F {
    fn handle(&self, modules: &Modules, url: UriParams, body: BodyParams) -> Vec<u8> {
        self(modules, url, body)
    }
}

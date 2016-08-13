use webapp::{Application, Responder, BodyResponder, Request};
use webapp::status::StatusCode;
use webapp::header::{Headers, ContentLength, Location};
use routes::{Routes, RouteResult};
use modules::Modules;

pub struct Clockwork<E> {
    modules: Modules,
    routes: Routes,
    error_handler: E,
}

impl<E: ErrorHandler> Clockwork<E> {
    pub fn new(modules: Modules, routes: Routes, error_handler: E) -> Self {
        Clockwork {
            modules: modules,
            routes: routes,
            error_handler: error_handler,
        }
    }
}

impl<E: ErrorHandler> Application for Clockwork<E> {
    fn on_request<R: Responder>(&self, request: Request, responder: R) {
        // Pass the request to the router
        let result = self.routes.handle(&self.modules, request.method, &request.path, request.body);

        // Check the route's result
        let (status, body, location): (StatusCode, Vec<u8>, Option<String>) = match result {
            RouteResult::Html(body) => (StatusCode::Ok, body.into(), None),
            RouteResult::Raw(body) => (StatusCode::Ok, body, None),
            RouteResult::Redirect(location) => (StatusCode::SeeOther, Vec::new(), Some(location)),
            RouteResult::Error(status) => {
                // An error happened, defer to the error handler
                let body = self.error_handler.handle(&self.modules, status);
                (status, body, None)
            }
        };

        // Construct the headers for the response
        let mut headers = Headers::new();
        headers.set(ContentLength(body.len() as u64));
        if let Some(location) = location {
            headers.set(Location(location));
        }

        // Send the response
        let mut body_responder = responder.start(status, headers);
        body_responder.send(body);
        body_responder.finish();
    }
}

pub trait ErrorHandler: Send + Sync + 'static {
    fn handle(&self, modules: &Modules, status: StatusCode) -> Vec<u8>;
}

impl<F: Fn(&Modules, StatusCode) -> Vec<u8> + Send + Sync + 'static> ErrorHandler for F {
    fn handle(&self, modules: &Modules, status: StatusCode) -> Vec<u8> {
        self(modules, status)
    }
}

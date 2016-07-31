use webapp::{Application, Responder, BodyResponder, Request};
use webapp::status::StatusCode;
use webapp::header::{Headers, ContentLength};
use routes::Routes;
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
        let result = self.routes.handle(&self.modules, request.method, &request.path);

        // Check the route's result
        let (status, body) = match result {
            Ok(body) => {
                // No error
                (StatusCode::Ok, body)
            }
            Err(status) => {
                // An error happened, defer to the error handler
                let body = self.error_handler.handle(&self.modules, status);
                (status, body)
            }
        };

        // Construct the headers for the response
        let mut headers = Headers::new();
        headers.set(ContentLength(body.len() as u64));

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

use webapp::{Application, Responder, BodyResponder, Request};
use webapp::status::StatusCode;
use webapp::header::{Headers, ContentLength};
use routes::Routes;
use modules::Modules;

pub struct Clockwork {
    modules: Modules,
    routes: Routes,
}

impl Clockwork {
    pub fn new(modules: Modules, routes: Routes) -> Self {
        Clockwork {
            modules: modules,
            routes: routes,
        }
    }
}

impl Application for Clockwork {
    fn on_request<R: Responder>(&self, request: Request, responder: R) {
        // Pass the request to the router
        let result = self.routes.handle(&self.modules, &request.path);

        // Send the HTTP header to respond with
        let mut headers = Headers::new();
        headers.set(ContentLength(result.len() as u64));
        let mut body = responder.start(StatusCode::Ok, headers);

        // Send the body of our response
        body.send(result);

        body.finish();
    }
}

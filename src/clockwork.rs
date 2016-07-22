use webapp::{Application, Responder, BodyResponder, Request, Header};
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

        // Reply to the request with the data from the route
        let header = Header {};
        let mut body = responder.start(header);
        body.send(result);
        body.finish();
    }
}

use routes::{RouteHandler, UriParams, BodyParams};
use modules::Modules;

pub fn model_handler<M: RouteModel, H: ModelRouteHandler<M>>(handler: H) -> ModelHandlerWrapper<M, H> {
    ModelHandlerWrapper {
        handler: handler,
        _model: ::std::marker::PhantomData,
    }
}

pub trait RouteModel: Send + Sync {
    fn from(url: UriParams, body: BodyParams) -> Self;
}

pub struct ModelHandlerWrapper<M: RouteModel, H: ModelRouteHandler<M>> {
    handler: H,
    _model: ::std::marker::PhantomData<M>,
}

impl<M: RouteModel, H: ModelRouteHandler<M>> RouteHandler for ModelHandlerWrapper<M, H> {
    fn handle(&self, modules: &Modules, url: UriParams, body: BodyParams) -> Vec<u8> {
        let model = M::from(url, body);
        self.handler.handle(modules, model)
    }
}

pub trait ModelRouteHandler<M: RouteModel>: Send + Sync {
    fn handle(&self, modules: &Modules, model: M) -> Vec<u8>;
}

impl<M: RouteModel, F: Fn(&Modules, M) -> Vec<u8> + Send + Sync> ModelRouteHandler<M> for F {
    fn handle(&self, modules: &Modules, model: M) -> Vec<u8> {
        self(modules, model)
    }
}

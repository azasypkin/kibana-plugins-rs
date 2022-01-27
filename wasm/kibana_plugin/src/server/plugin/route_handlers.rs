use kibana_core_types::server::{Request, RequestContext, RequestHandler, ResponseFactory};
use wasm_bindgen::prelude::*;

pub struct RouteHandlers {
    handlers: Vec<RequestHandler>,
}

impl RouteHandlers {
    pub fn new() -> Self {
        Self { handlers: vec![] }
    }

    pub fn create_handler<F: 'static>(&mut self, f: F) -> &RequestHandler
    where
        F: FnMut(RequestContext, Request, ResponseFactory) -> js_sys::Promise,
    {
        self.handlers.push(Closure::wrap(Box::new(f)
            as Box<
                dyn FnMut(RequestContext, Request, ResponseFactory) -> js_sys::Promise,
            >));
        &self.handlers[self.handlers.len() - 1]
    }
}

mod request;
mod request_context;
mod request_handler;
mod response;
mod response_factory;
mod response_options;
mod route_config;

pub use self::{
    request::Request, request_context::RequestContext, request_handler::RequestHandler,
    response::Response, response_factory::ResponseFactory, response_options::ResponseOptions,
    route_config::RouteConfig,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Router;

    #[wasm_bindgen(method)]
    pub fn post(this: &Router, route_config: RouteConfig, route_handler: &RequestHandler);
}

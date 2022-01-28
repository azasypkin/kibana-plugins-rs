mod request;
mod request_context;
mod request_handler;
mod response;
mod response_factory;
mod response_options;
mod route_config;
mod route_schema;

use self::route_config::{RouteConfigWithValidation, RouteConfigWithoutValidation};
pub use self::{
    request::Request,
    request_context::{CoreRequestContext, ElasticsearchRequestContext, RequestContext},
    request_handler::RequestHandler,
    response::Response,
    response_factory::ResponseFactory,
    response_options::ResponseOptions,
    route_config::RouteConfig,
    route_schema::RouteSchema,
};
use either::Either;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Router;

    #[wasm_bindgen(method, catch, js_name = post)]
    fn external_post_without_validation(
        this: &Router,
        route_config: RouteConfigWithoutValidation,
        route_handler: &RequestHandler,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = post)]
    fn external_post(
        this: &Router,
        route_config: RouteConfigWithValidation,
        route_handler: &RequestHandler,
    ) -> Result<(), JsValue>;
}

impl Router {
    pub fn post(
        &self,
        route_config: RouteConfig,
        route_handler: &RequestHandler,
    ) -> Result<(), JsValue> {
        match route_config.build()? {
            Either::Left(config) => self.external_post(config, route_handler),
            Either::Right(config) => self.external_post_without_validation(config, route_handler),
        }
    }
}

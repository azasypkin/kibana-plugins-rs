use super::Response;
use crate::server::http::router::response_options::ResponseOptions;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type ResponseFactory;

    #[wasm_bindgen(method)]
    pub fn ok(this: &ResponseFactory) -> Response;

    #[wasm_bindgen(method, js_name = ok)]
    pub fn ok_with_options(this: &ResponseFactory, options: ResponseOptions) -> Response;
}

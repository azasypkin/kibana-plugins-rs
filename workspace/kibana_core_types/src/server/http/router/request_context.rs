use wasm_bindgen::prelude::*;

mod core_request_context;
mod elasticsearch_request_context;

pub use self::{
    core_request_context::CoreRequestContext,
    elasticsearch_request_context::ElasticsearchRequestContext,
};

#[wasm_bindgen]
extern "C" {
    pub type RequestContext;

    #[wasm_bindgen(method, getter)]
    pub fn core(this: &RequestContext) -> CoreRequestContext;
}

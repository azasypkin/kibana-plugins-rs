use crate::server::elasticsearch::ScopedElasticsearchClient;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type ElasticsearchRequestContext;

    #[wasm_bindgen(method, getter)]
    pub fn client(this: &ElasticsearchRequestContext) -> ScopedElasticsearchClient;
}

use super::ElasticsearchRequestContext;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type CoreRequestContext;

    #[wasm_bindgen(method, getter)]
    pub fn elasticsearch(this: &CoreRequestContext) -> ElasticsearchRequestContext;
}

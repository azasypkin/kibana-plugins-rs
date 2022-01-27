use super::ElasticsearchClient;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type ScopedElasticsearchClient;

    #[wasm_bindgen(method, getter, js_name = asCurrentUser)]
    pub fn as_current_user(this: &ScopedElasticsearchClient) -> ElasticsearchClient;
}

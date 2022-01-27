use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type ElasticsearchClient;
    pub type ElasticsearchClientSecurity;

    #[wasm_bindgen(method, getter)]
    pub fn security(this: &ElasticsearchClient) -> ElasticsearchClientSecurity;

    #[wasm_bindgen(method, catch)]
    pub async fn authenticate(this: &ElasticsearchClientSecurity) -> Result<JsValue, JsValue>;
}

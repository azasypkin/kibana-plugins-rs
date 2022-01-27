use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct AuthenticationInfo {
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ElasticsearchResponse<T> {
    body: T,
}

#[wasm_bindgen]
extern "C" {
    pub type ElasticsearchClient;
    pub type ElasticsearchClientSecurity;

    #[wasm_bindgen(method, getter)]
    pub fn security(this: &ElasticsearchClient) -> ElasticsearchClientSecurity;

    #[wasm_bindgen(method, catch, js_name = authenticate)]
    async fn external_authenticate(this: &ElasticsearchClientSecurity) -> Result<JsValue, JsValue>;
}

impl ElasticsearchClientSecurity {
    pub async fn authenticate(&self) -> Result<AuthenticationInfo, JsValue> {
        // WORKAROUND: wasm_bindgen can only return `JsValue` for functions that return `Promise`
        // and we should do deserialization manually.
        self.external_authenticate()
            .await
            .and_then(|js_value| {
                JsValue::into_serde::<ElasticsearchResponse<AuthenticationInfo>>(&js_value)
                    .map_err(|serialize_error| JsValue::from(serialize_error.to_string()))
            })
            .map(|response| response.body)
    }
}

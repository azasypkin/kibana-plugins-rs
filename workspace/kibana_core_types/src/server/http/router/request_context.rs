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

    #[wasm_bindgen(method, catch, getter, js_name = core)]
    async fn external_core(this: &RequestContext) -> Result<JsValue, JsValue>;
}

impl RequestContext {
    pub async fn core(&self) -> Result<CoreRequestContext, JsValue> {
        // WORKAROUND: wasm_bindgen can only return `JsValue` for functions that return `Promise`
        // and we should do deserialization manually.
        self.external_core().await.map(|core| core.into())
    }
}

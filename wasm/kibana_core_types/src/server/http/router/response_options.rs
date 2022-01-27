use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct ResponseOptions {
    pub body: Option<String>,
}

impl ResponseOptions {
    pub fn with_body(body: impl AsRef<str>) -> Self {
        Self {
            body: Some(body.as_ref().to_string()),
        }
    }
}

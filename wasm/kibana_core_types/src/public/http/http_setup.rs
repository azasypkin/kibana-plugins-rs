use super::base_path::BasePath;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type HttpSetup;

    #[wasm_bindgen(method, getter, js_name = basePath)]
    pub fn base_path(this: &HttpSetup) -> BasePath;
}

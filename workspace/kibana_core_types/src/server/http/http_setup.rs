use super::{BasePath, Router};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type HttpSetup;

    #[wasm_bindgen(method, getter, js_name = basePath)]
    pub fn base_path(this: &HttpSetup) -> BasePath;

    #[wasm_bindgen(method, js_name = createRouter)]
    pub fn create_router(this: &HttpSetup) -> Router;
}

use crate::server::Logger;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type LoggerFactory;

    #[wasm_bindgen(method)]
    pub fn get(this: &LoggerFactory) -> Logger;

    #[wasm_bindgen(method, js_name = get)]
    pub fn get_with_context(this: &LoggerFactory, context: &str) -> Logger;
}

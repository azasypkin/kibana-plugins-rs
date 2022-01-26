use super::super::logging::LoggerFactory;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type PluginInitializerContext;

    #[wasm_bindgen(method, getter)]
    pub fn logger(this: &PluginInitializerContext) -> LoggerFactory;
}

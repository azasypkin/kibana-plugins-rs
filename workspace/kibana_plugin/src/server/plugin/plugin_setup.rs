use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PluginSetup {}

#[wasm_bindgen]
impl PluginSetup {
    pub(crate) fn new() -> Self {
        PluginSetup {}
    }
}

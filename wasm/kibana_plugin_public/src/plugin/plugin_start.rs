use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Plugin start contract.
pub struct PluginStart {}

#[wasm_bindgen]
impl PluginStart {
    pub(crate) fn new() -> Self {
        PluginStart {}
    }
}

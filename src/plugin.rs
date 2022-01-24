use crate::plugin_setup::PluginSetup;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Plugin {}

#[wasm_bindgen]
impl Plugin {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn setup(&self) -> PluginSetup {
        PluginSetup::new()
    }
}

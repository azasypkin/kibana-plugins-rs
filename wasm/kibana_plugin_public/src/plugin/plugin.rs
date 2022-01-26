use crate::plugin::{PluginSetup, PluginStart};
use kibana_core_types::public::CoreSetup;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct Plugin {}

#[wasm_bindgen]
impl Plugin {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn setup(&self, core: CoreSetup) -> Result<PluginSetup, JsValue> {
        console::log_1(
            &format!(
                "[FROM WASM] Setting up plugin (base path {}).",
                core.http().base_path().get()
            )
            .into(),
        );
        Ok(PluginSetup::new())
    }

    pub fn start(&self) -> Result<PluginStart, JsValue> {
        console::log_1(&"[FROM WASM] Starting plugin.".into());
        Ok(PluginStart::new())
    }
}

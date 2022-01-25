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

    pub fn setup(&self, core: CoreSetup) -> PluginSetup {
        console::log_1(
            &format!(
                "Setting up plugin (base path {}).",
                core.http().base_path().get()
            )
            .into(),
        );
        PluginSetup::new(core)
    }

    pub fn start(&self) -> PluginStart {
        console::log_1(&"Starting plugin.".into());
        PluginStart::new()
    }
}

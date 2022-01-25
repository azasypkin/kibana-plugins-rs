use crate::plugin::{PluginSetup, PluginStart};
use kibana_core_types::public::http::HttpSetup;
use wasm_bindgen::prelude::*;

use web_sys::console;

#[wasm_bindgen]
pub struct Plugin {
    http: HttpSetup,
}

#[wasm_bindgen]
impl Plugin {
    #[wasm_bindgen(constructor)]
    pub fn new(http: HttpSetup) -> Self {
        Self { http }
    }

    pub fn setup(&self) -> PluginSetup {
        console::log_1(&"Setting up plugin.".into());
        PluginSetup::new()
    }

    pub fn start(&self) -> PluginStart {
        console::log_1(&"Starting plugin.".into());
        PluginStart::new()
    }
}

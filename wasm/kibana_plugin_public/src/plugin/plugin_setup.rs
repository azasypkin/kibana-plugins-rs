use kibana_core_types::public::CoreSetup;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PluginSetup {
    core: CoreSetup,
}

#[wasm_bindgen]
impl PluginSetup {
    pub(crate) fn new(core: CoreSetup) -> Self {
        PluginSetup { core }
    }
}

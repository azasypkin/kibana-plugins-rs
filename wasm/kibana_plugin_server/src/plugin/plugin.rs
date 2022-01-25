use crate::plugin::{PluginSetup, PluginStart};
use kibana_core_types::server::logging::Logger;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Plugin {
    logger: Logger,
}

#[wasm_bindgen]
impl Plugin {
    #[wasm_bindgen(constructor)]
    pub fn new(logger: Logger) -> Self {
        Self { logger }
    }

    pub fn setup(&self) -> PluginSetup {
        self.logger.debug("Setting up plugin.");
        PluginSetup::new()
    }

    pub fn start(&self) -> PluginStart {
        self.logger.debug("Starting plugin.");
        PluginStart::new()
    }
}

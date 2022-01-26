use crate::plugin::{PluginSetup, PluginStart};
use kibana_core_types::server::{packages::kbn_i18n, Logger};
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

    pub fn setup(&self) -> Result<PluginSetup, JsValue> {
        self.logger.info(&format!(
            "Setting up plugin ({}).",
            kbn_i18n::translate(
                "exampleRs.welcomeMessage",
                kbn_i18n::I18nParams::new("Welcome {name}!".to_string())
                    .with_values([("name", "Kibana")].iter().cloned().collect()),
            )?
        ));

        Ok(PluginSetup::new())
    }

    pub fn start(&self) -> Result<PluginStart, JsValue> {
        self.logger.debug("Starting plugin.");
        Ok(PluginStart::new())
    }
}

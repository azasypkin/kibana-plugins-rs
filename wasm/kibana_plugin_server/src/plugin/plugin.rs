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
        let i18n_params = kbn_i18n::I18nParams::new("My localized name is {name}".to_string())
            .with_values([("name", "Kibana")].iter().cloned().collect())
            .map_err(|_| JsError::new("Failed to serialize i18n parameters."))?;

        self.logger.info(&format!(
            "Setting up plugin with message: {}.",
            kbn_i18n::translate("external.rust.name", i18n_params)
        ));

        Ok(PluginSetup::new())
    }

    pub fn start(&self) -> PluginStart {
        self.logger.debug("Starting plugin.");
        PluginStart::new()
    }
}

use crate::server::plugin::{PluginSetup, PluginStart};
use kibana_core_types::server::{packages::kbn_i18n, PluginInitializerContext};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Plugin {
    context: PluginInitializerContext,
}

#[wasm_bindgen]
impl Plugin {
    #[wasm_bindgen(constructor)]
    pub fn new(context: PluginInitializerContext) -> Self {
        Self { context }
    }

    pub fn setup(&self) -> Result<PluginSetup, JsValue> {
        let logger = self.context.logger().get_with_context("wasm");
        logger.info(&format!(
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
        self.context
            .logger()
            .get_with_context("wasm")
            .debug("Starting plugin.");
        Ok(PluginStart::new())
    }
}

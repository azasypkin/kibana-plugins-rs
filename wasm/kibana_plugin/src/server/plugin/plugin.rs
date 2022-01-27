use super::route_handlers::RouteHandlers;
use crate::server::plugin::{PluginSetup, PluginStart};
use kibana_core_types::server::{
    packages::kbn_i18n, CoreSetup, Logger, PluginInitializerContext, ResponseOptions, RouteConfig,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Plugin {
    logger: Logger,
    route_handlers: RouteHandlers,
}

#[wasm_bindgen]
impl Plugin {
    #[wasm_bindgen(constructor)]
    pub fn new(context: PluginInitializerContext) -> Self {
        Self {
            logger: context.logger().get_with_context("wasm"),
            route_handlers: RouteHandlers::new(),
        }
    }

    pub fn setup(&mut self, core: CoreSetup) -> Result<PluginSetup, JsValue> {
        self.logger.debug("Setting up plugin.");

        let router = core.http().create_router();
        router.post(
            RouteConfig::new("/api/wasm"),
            self.route_handlers.create_handler(|context, _, res| {
                let es_client = context
                    .core()
                    .elasticsearch()
                    .client()
                    .as_current_user()
                    .security();

                /*wasm_bindgen_futures::spawn_local((|| async {
                    let promise: JsValue = es_client.authenticate().await.unwrap();
                })());*/

                let i18n_params = kbn_i18n::I18nParams::new("Welcome {name}!".to_string())
                    .with_values([("name", "Kibana")].iter().cloned().collect());
                kbn_i18n::translate("exampleRs.welcomeMessage", i18n_params).map(
                    |translated_string| {
                        res.ok_with_options(ResponseOptions::with_body(format!(
                            "Response: {}.",
                            translated_string
                        )))
                    },
                )
            }),
        );

        Ok(PluginSetup::new())
    }

    pub fn start(&self) -> Result<PluginStart, JsValue> {
        self.logger.debug("Starting plugin.");
        Ok(PluginStart::new())
    }
}

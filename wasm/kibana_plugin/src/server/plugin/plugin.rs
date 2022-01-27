use super::{route_handlers::RouteHandlers, PluginSetup, PluginStart};
use crate::common::find_similarity;
use kibana_core_types::server::{
    packages::kbn_i18n, AuthenticationInfo, CoreSetup, Logger, PluginInitializerContext,
    ResponseOptions, RouteConfig,
};
use serde_json::json;
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
                wasm_bindgen_futures::future_to_promise(async move {
                    // Retrieve current user information from Elasticsearch.
                    let es_client = context.core().elasticsearch().client().as_current_user();
                    let current_user: AuthenticationInfo =
                        es_client.security().authenticate().await?;

                    // Use kbn/i18n package to localize message.
                    let i18n_params = kbn_i18n::I18nParams::new("Welcome {name}!".to_string())
                        .with_values(
                            [("name", current_user.username.as_ref())]
                                .iter()
                                .cloned()
                                .collect(),
                        );
                    let i18n_comment = kbn_i18n::translate("exampleRs.welcomeMessage", i18n_params)?;

                    // Prepare response struct.
                    let response = res.ok_with_options(ResponseOptions::with_body(
                        json!({ "comment": i18n_comment, "similarity": find_similarity("k", "k").value })
                            .to_string(),
                    ));

                    // Turn response into a `JSValue`.
                    Ok(response.into())
                })
            }),
        );

        Ok(PluginSetup::new())
    }

    pub fn start(&self) -> Result<PluginStart, JsValue> {
        self.logger.debug("Starting plugin.");
        Ok(PluginStart::new())
    }
}

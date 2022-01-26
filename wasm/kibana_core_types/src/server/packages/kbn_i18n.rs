use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct I18nParams {
    default_message: String,
    values: Option<JsValue>,
}

impl I18nParams {
    pub fn new(default_message: impl AsRef<str>) -> Self {
        Self {
            default_message: default_message.as_ref().to_owned(),
            values: None,
        }
    }

    pub fn with_values(self, values: HashMap<&str, &str>) -> Result<Self, ()> {
        JsValue::from_serde(&values)
            .map_err(|_| ())
            .map(|values| Self {
                values: Some(values),
                ..self
            })
    }
}

#[wasm_bindgen]
impl I18nParams {
    #[wasm_bindgen(getter, js_name = defaultMessage)]
    pub fn default_message(&self) -> String {
        self.default_message.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn values(&self) -> JsValue {
        self.values.clone().into()
    }
}

#[wasm_bindgen(module = "@kbn/i18n")]
extern "C" {
    #[wasm_bindgen(js_namespace = i18n)]
    pub fn translate(id: &str, params: I18nParams) -> String;
}

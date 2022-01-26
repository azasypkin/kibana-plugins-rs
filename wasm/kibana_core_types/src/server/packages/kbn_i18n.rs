use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub struct I18nParams<'k, 'v> {
    default_message: String,
    values: Option<HashMap<&'k str, &'v str>>,
}

impl<'k, 'v> I18nParams<'k, 'v> {
    pub fn new(default_message: impl AsRef<str>) -> Self {
        Self {
            default_message: default_message.as_ref().to_owned(),
            values: None,
        }
    }

    pub fn with_values(self, values: HashMap<&'k str, &'v str>) -> Self {
        Self {
            values: Some(values),
            ..self
        }
    }
}

#[wasm_bindgen(module = "@kbn/i18n")]
extern "C" {
    #[wasm_bindgen(js_namespace = i18n, js_name = translate, catch)]
    fn extern_translate(id: &str, params: &JsValue) -> Result<String, JsValue>;
}

pub fn translate(id: &str, params: I18nParams) -> Result<String, JsValue> {
    let js_params = js_sys::Object::new();
    js_sys::Reflect::set(
        &js_params,
        &"defaultMessage".into(),
        &params.default_message.into(),
    )?;

    if let Some(values) = params.values {
        let js_values = js_sys::Object::new();
        for (key, value) in values {
            js_sys::Reflect::set(&js_values, &key.into(), &value.into())?;
        }

        js_sys::Reflect::set(&js_params, &"values".into(), &js_values)?;
    }

    extern_translate(id, &js_params)
}

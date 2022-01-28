use crate::server::packages::kbn_config_schema::{SchemaBlock, Type};
use wasm_bindgen::JsValue;

#[derive(Default)]
pub struct RouteSchema {
    body: Option<Type>,
    params: Option<Type>,
    query: Option<Type>,
}

impl RouteSchema {
    pub fn body(self, schema: impl SchemaBlock) -> Self {
        Self {
            body: Some(schema.into()),
            ..self
        }
    }

    pub fn params(self, schema: impl SchemaBlock) -> Self {
        Self {
            params: Some(schema.into()),
            ..self
        }
    }

    pub fn query(self, schema: impl SchemaBlock) -> Self {
        Self {
            query: Some(schema.into()),
            ..self
        }
    }

    pub(crate) fn build(self) -> Result<js_sys::Object, JsValue> {
        let js_validate = js_sys::Object::create(&JsValue::null().into());
        if let Some(body) = self.body {
            js_sys::Reflect::set(&js_validate, &"body".into(), &body.into())?;
        }

        if let Some(params) = self.params {
            js_sys::Reflect::set(&js_validate, &"params".into(), &params.into())?;
        }

        if let Some(query) = self.query {
            js_sys::Reflect::set(&js_validate, &"query".into(), &query.into())?;
        }

        Ok(js_validate)
    }
}

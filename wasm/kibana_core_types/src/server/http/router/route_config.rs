use super::RouteSchema;
use either::Either;
use wasm_bindgen::prelude::*;

pub struct RouteConfig {
    pub path: String,
    pub schema: Option<RouteSchema>,
}

impl RouteConfig {
    pub fn new(path: impl AsRef<str>) -> Self {
        Self {
            path: path.as_ref().to_string(),
            schema: None,
        }
    }

    pub fn with_schema(self, schema: RouteSchema) -> Self {
        Self {
            schema: Some(schema),
            ..self
        }
    }

    pub(crate) fn build(
        self,
    ) -> Result<Either<RouteConfigWithValidation, RouteConfigWithoutValidation>, JsValue> {
        if let Some(schema) = self.schema {
            schema.build().map(|validate| {
                Either::Left(RouteConfigWithValidation {
                    path: self.path,
                    validate,
                })
            })
        } else {
            Ok(Either::Right(RouteConfigWithoutValidation {
                path: self.path,
                validate: false,
            }))
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[allow(dead_code)]
pub(crate) struct RouteConfigWithoutValidation {
    pub path: String,
    pub validate: bool,
}

#[wasm_bindgen(getter_with_clone)]
#[allow(dead_code)]
pub(crate) struct RouteConfigWithValidation {
    pub path: String,
    pub validate: js_sys::Object,
}

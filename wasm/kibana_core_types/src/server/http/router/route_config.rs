use wasm_bindgen::prelude::*;

#[wasm_bindgen(getter_with_clone)]
pub struct RouteConfig {
    pub path: String,
    pub validate: bool,
}

impl RouteConfig {
    pub fn new(path: impl AsRef<str>) -> Self {
        Self {
            path: path.as_ref().to_string(),
            validate: false,
        }
    }
}

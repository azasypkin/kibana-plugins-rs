use super::super::HttpSetup;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type CoreSetup;

    #[wasm_bindgen(method, getter)]
    pub fn http(this: &CoreSetup) -> HttpSetup;
}

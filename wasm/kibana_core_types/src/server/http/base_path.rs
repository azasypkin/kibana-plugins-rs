use super::Request;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type BasePath;

    #[wasm_bindgen(method)]
    pub fn get(this: &BasePath, request: Request) -> String;
}

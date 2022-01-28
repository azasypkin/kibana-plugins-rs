use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Logger;

    #[wasm_bindgen(method)]
    pub fn info(this: &Logger, message: &str);

    #[wasm_bindgen(method)]
    pub fn debug(this: &Logger, message: &str);
}

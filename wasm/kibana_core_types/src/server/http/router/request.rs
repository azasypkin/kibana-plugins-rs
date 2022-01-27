use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Request;

    #[wasm_bindgen(method, getter, js_name = body)]
    fn external_body(this: &Request) -> JsValue;
}

impl Request {
    pub fn body<T: for<'a> serde::de::Deserialize<'a>>(&self) -> Option<T> {
        let body = self.external_body();
        if body == JsValue::NULL || body == JsValue::UNDEFINED {
            None
        } else {
            match JsValue::into_serde(&body) {
                Ok(body) => Some(body),
                Err(_) => None,
            }
        }
    }
}

use super::{Request, RequestContext, ResponseFactory};
use wasm_bindgen::prelude::*;

pub type RequestHandler =
    Closure<dyn FnMut(RequestContext, Request, ResponseFactory) -> js_sys::Promise>;

use super::{Request, RequestContext, Response, ResponseFactory};
use wasm_bindgen::prelude::*;

pub type RequestHandler =
    Closure<dyn FnMut(RequestContext, Request, ResponseFactory) -> Result<Response, JsValue>>;

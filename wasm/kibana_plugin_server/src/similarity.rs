mod similarity_method;

use serde::{Deserialize, Serialize};
pub use similarity_method::SimilarityMethod;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct Similarity {
    pub value: f64,
    pub method: SimilarityMethod,
}

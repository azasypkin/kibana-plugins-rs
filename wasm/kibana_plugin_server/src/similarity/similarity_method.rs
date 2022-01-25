use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum SimilarityMethod {
    Levenshtein = 0,
    Other = 1,
}

use crate::similarity::{Similarity, SimilarityMethod};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PluginSetup {}

#[wasm_bindgen]
impl PluginSetup {
    pub(crate) fn new() -> Self {
        PluginSetup {}
    }

    #[wasm_bindgen(js_name = findSimilarity)]
    pub fn find_similarity(&self, string_a: &str, string_b: &str) -> Similarity {
        Similarity {
            value: strsim::normalized_levenshtein(string_a, string_b),
            method: SimilarityMethod::Levenshtein,
        }
    }
}

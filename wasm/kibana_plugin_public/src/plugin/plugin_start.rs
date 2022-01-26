use kibana_plugin_common::similarity::{find_similarity, Similarity};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Plugin start contract.
pub struct PluginStart {}

#[wasm_bindgen]
impl PluginStart {
    pub(crate) fn new() -> Self {
        PluginStart {}
    }

    /// Calculates a normalized score of the Levenshtein algorithm between 0.0 and 1.0 (inclusive),
    /// where 1.0 means the strings are the same.
    #[wasm_bindgen(js_name = findSimilarity)]
    pub fn find_similarity(&self, string_a: &str, string_b: &str) -> Similarity {
        find_similarity(string_a, string_b)
    }
}

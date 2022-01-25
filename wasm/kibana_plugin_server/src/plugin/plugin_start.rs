use crate::similarity::{Similarity, SimilarityMethod};
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
        Similarity {
            value: strsim::normalized_levenshtein(string_a, string_b),
            method: SimilarityMethod::Levenshtein,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PluginStart, Similarity, SimilarityMethod};

    #[test]
    fn properly_calculates_similarity() {
        let plugin_start = PluginStart::new();
        assert_eq!(
            plugin_start.find_similarity("Elasticsearch", "Kibana"),
            Similarity {
                value: 0.15384615384615385,
                method: SimilarityMethod::Levenshtein
            }
        );
        assert_eq!(
            plugin_start.find_similarity("Kabana", "Kibana"),
            Similarity {
                value: 0.8333333333333334,
                method: SimilarityMethod::Levenshtein
            }
        );
    }
}

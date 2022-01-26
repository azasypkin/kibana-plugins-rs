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

/// Calculates a normalized score of the Levenshtein algorithm between 0.0 and 1.0 (inclusive),
/// where 1.0 means the strings are the same.
pub fn find_similarity(string_a: &str, string_b: &str) -> Similarity {
    Similarity {
        value: strsim::normalized_levenshtein(string_a, string_b),
        method: SimilarityMethod::Levenshtein,
    }
}

#[cfg(test)]
mod tests {
    use super::{find_similarity, Similarity, SimilarityMethod};

    #[test]
    fn properly_calculates_similarity() {
        assert_eq!(
            find_similarity("Elasticsearch", "Kibana"),
            Similarity {
                value: 0.15384615384615385,
                method: SimilarityMethod::Levenshtein
            }
        );
        assert_eq!(
            find_similarity("Kabana", "Kibana"),
            Similarity {
                value: 0.8333333333333334,
                method: SimilarityMethod::Levenshtein
            }
        );
    }
}

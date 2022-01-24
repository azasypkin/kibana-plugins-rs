mod plugin;
mod plugin_setup;
mod similarity;
mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

pub use crate::{
    plugin::Plugin,
    plugin_setup::PluginSetup,
    similarity::{Similarity, SimilarityMethod},
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn similarity(string_a: &str, string_b: &str) -> f64 {
    console::log_1(&JsValue::from(format!(
        "Comparing {} and {}",
        string_a, string_b
    )));

    strsim::normalized_levenshtein(string_a, string_b)
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}

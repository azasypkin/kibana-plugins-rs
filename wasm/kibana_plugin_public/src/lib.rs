mod plugin;
mod utils;

use wasm_bindgen::prelude::*;

pub use crate::plugin::{Plugin, PluginSetup, PluginStart};
pub use kibana_plugin_common::similarity::{Similarity, SimilarityMethod};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}

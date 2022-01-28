#![deny(warnings)]

mod common;
mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "public")]
pub mod public;

pub use common::{Similarity, SimilarityMethod};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}

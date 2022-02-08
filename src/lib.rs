mod utils;

use wasm_bindgen::prelude::*;
use lz77::lz77::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compress(data: &[u8]) -> String {
    let compressed = lz77_compress(data);
    base64::encode(compressed)
}

#[wasm_bindgen]
pub fn decompress(data: &[u8]) -> String {
    let decompressed = lz77_decompress(data);
    base64::encode(decompressed)
}

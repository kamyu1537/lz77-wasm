mod utils;

use lz77::lz77::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compress(data: &[u8]) -> Box<[u8]> {
    let compressed = lz77_compress(data);
    compressed.into_boxed_slice()
}

#[wasm_bindgen]
pub fn decompress(data: &[u8]) -> Box<[u8]> {
    let decompressed = lz77_decompress(data);
    decompressed.into_boxed_slice()
}

pub mod geom;
pub mod particle;
pub mod simulation;
pub mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

pub(crate) const SECRET_KEY: &[u8] = match option_env!("SECRET_KEY") {
    Some(value) => value,
    None => "The Magic Words are Squeamish Ossifrage",
}
.as_bytes();

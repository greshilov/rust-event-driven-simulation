use wasm_bindgen::prelude::*;

use hmac::Hmac;
use sha2::Sha256;

#[wasm_bindgen]
#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[allow(dead_code)]
pub fn compare_floats(a: f64, b: f64) {
    const EPS: f64 = 1e-12;
    assert!((a - b).abs() < EPS, "{} != {}", a, b);
}

pub type HmacSha256 = Hmac<Sha256>;

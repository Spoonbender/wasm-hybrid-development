use primes::{PrimeSet, Sieve};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "doForPrimes")]
pub fn do_for_primes(amount: u32, handler: &js_sys::Function) {
    let this = JsValue::null();
    let mut primes = Sieve::new();
    for (_ix, n) in primes.iter().enumerate().take(amount as usize) {
        let prime = JsValue::from_f64(n as f64);
        let _ = handler.call1(&this, &prime);
    }
}

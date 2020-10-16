use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "printFibonacci")]
pub fn print_fibonacci() {
    let mut x: u64 = 0;
    let mut y: u64 = 1;

    let sequence_generator = &mut || {
        let current = x + y;
        x = y;
        y = current;
        return current;
    };

    printSequence(sequence_generator);
}

#[wasm_bindgen(raw_module = "../demos/rust_closures.js")]
extern "C" {
    #[wasm_bindgen]
    fn printSequence(next: &mut dyn FnMut() -> u64);
}

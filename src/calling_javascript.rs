use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet_redux(name: &str) {
    let msg = format!("Hello from WASM, {}!", name);
    alert(&msg);
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

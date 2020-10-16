use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn meow() {
    match open_schrodingers_box() {
        Ok(value) => console_log(&format!(
            "Schrödinger's cat says: {}",
            value.as_string().unwrap()
        )),
        Err(e) => console_warn(&format!("I got a JavaScript error! {:?}", e)),
    }
}

#[wasm_bindgen(raw_module = "../demos/error_handling.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = "openSchrodingersBox")]
    fn open_schrodingers_box() -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    fn console_log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = "warn")]
    fn console_warn(s: &str);
}

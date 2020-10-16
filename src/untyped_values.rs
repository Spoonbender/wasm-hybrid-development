use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn print_my_keys(input: &JsValue) -> String {
    let properties: Array = js_sys::Reflect::own_keys(input).unwrap();

    let mut result = String::new();

    for key in properties.iter() {
        result.push_str(&format!(
            "{:?} = {:?}\n",
            key,
            js_sys::Reflect::get(input, &key).unwrap()
        ));
    }

    result
}

pub mod async_functions;
pub mod calling_javascript;
pub mod dom_manipulation;
pub mod duck_typing;
pub mod error_handling;
pub mod hello_world;
pub mod js_closures;
pub mod rust_closures;
pub mod typescripting;
pub mod untyped_values;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub async fn get_file_via_http(url: String) -> Result<JsValue, JsValue> {
    match async_functions::http_get(&url).await {
        Ok(text) => Ok(text.into()),
        Err(e) => Err(JsValue::from_str(&format!("Failed getting file via HTTP! {:?}", e))),
    }
}
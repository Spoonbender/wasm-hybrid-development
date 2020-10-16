use wasm_bindgen::prelude::*;

#[wasm_bindgen()]
pub fn add_element(parent_id: &str, text: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let demo_div = document.get_element_by_id(parent_id).unwrap();
    let new_element = document.create_element("p")?;
    new_element.set_inner_html(text);
    demo_div.append_child(&new_element)?;

    Ok(())
}

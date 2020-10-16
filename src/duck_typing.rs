use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Duck;

    #[wasm_bindgen(structural, method)]
    pub fn quack(this: &Duck) -> String;
}

#[wasm_bindgen]
pub fn treat_like_a_duck(duck: &Duck) {
    let mut test = String::new();
    for _ in 1..=3 {
        test.push_str(&duck.quack());
        test.push(' ');
    }

    console_log(&test);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    fn console_log(s: &str);
}

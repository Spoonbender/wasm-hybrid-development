use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen(js_name = "getWebAssemblyLanguages")]
pub async fn get_wasm_langs(url: String) -> Result<String, JsValue> {
    let text = match http_get(&url).await {
        Ok(text) => text,
        Err(e) => {
            return Err(JsValue::from_str(&format!(
                "Failed getting file via HTTP! {:?}",
                e
            )))
        }
    };

    let result = text
        .lines()
        .skip_while(|x| !x.starts_with("## Contents"))
        .skip(1)
        .take_while(|x| x.starts_with(":"))
        .map(|x| parse_line(x))
        .map(|(status, name)| format!("{}: {:?}\n", name, status))
        .fold(String::new(), |mut builder, line| {
            builder.push_str(&line);
            builder
        });

    Ok(result.into())
}

pub async fn http_get(url: &str) -> reqwest::Result<String> {
    reqwest::get(url).await?.text().await
}

#[derive(Debug)]
enum LangStatus {
    WIP,
    Unstable,
    Stable,
}

fn parse_line(line: &str) -> (LangStatus, &str) {
    let status = match line.split(":").skip(1).next().unwrap() {
        "hatched_chick" => LangStatus::Stable,
        "hatching_chick" => LangStatus::Unstable,
        _ => LangStatus::WIP,
    };

    let name = line
        .split("[")
        .skip(1)
        .next()
        .unwrap()
        .split("]")
        .next()
        .unwrap();

    (status, name)
}

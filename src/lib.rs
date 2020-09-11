use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log("OK !");
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn return_string() -> String {
    "hello".into()
}

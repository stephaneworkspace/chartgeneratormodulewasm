use chartgeneratorsvg::chord::FretID;
use chartgeneratorsvg::interface::{InterfaceWasm, TraitChord, TraitScale};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct UkuleleWasm {
    theme: String, // TODO better later
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
/*
#[wasm_bindgen]
pub fn greet(name: &str) {
    log("OK !");
    alert(&format!("Hello, {}!", name));
}*/
#[wasm_bindgen(catch)]
impl UkuleleWasm {
    pub fn new() -> Self {
        Self {
            theme: "dark".to_string(), // TODO better later
        }
    }

    pub fn chord_list_multiple(
        &self,
        note: &str,
        fret_position: FretID,
    ) -> String {
        InterfaceWasm::chord_list(note, fret_position).into()
    }

    pub fn scale_list_select(&self) -> String {
        InterfaceWasm::scale_list_wasm().into()
    }

    #[wasm_bindgen(catch)]
    pub fn scale_unique_svg(
        &self,
        scale_short: &str,
        tonic: &str,
    ) -> Result<String, JsValue> {
        match InterfaceWasm::scale_print(scale_short, tonic) {
            Ok(res) => Ok(res),
            Err(e) => Err(JsValue::from_str(e.name.as_str())),
        }
    }
    pub fn scale_chord_list_multiple(
        &self,
        scale_short: &str,
        tonic: &str,
        fret_position: FretID,
    ) -> String {
        InterfaceWasm::scale_chord_list(scale_short, tonic, fret_position)
            .into()
    }
}

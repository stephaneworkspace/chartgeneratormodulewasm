use chartgeneratorsvg::chord::FretID;
use chartgeneratorsvg::interface::{InterfaceWasm, TraitChord, TraitScale};
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
/*
#[wasm_bindgen]
pub fn greet(name: &str) {
    log("OK !");
    alert(&format!("Hello, {}!", name));
}*/

#[wasm_bindgen]
pub fn chord_list_multiple(note: &str, fret_position: FretID) -> String {
    InterfaceWasm::chord_list(note, fret_position).into()
}

#[wasm_bindgen]
pub fn scale_list_select() -> String {
    InterfaceWasm::scale_list_wasm().into()
}

#[wasm_bindgen]
pub fn scale_unique_svg(scale_short: &str, tonic: &str) -> String {
    InterfaceWasm::scale_print_wasm(scale_short, tonic).into()
}

#[wasm_bindgen]
pub fn scale_chord_list_multiple(
    scale_short: &str,
    tonic: &str,
    fret_position: FretID,
) -> String {
    InterfaceWasm::scale_chord_list(scale_short, tonic, fret_position).into()
}

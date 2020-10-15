//extern crate libc;

extern crate base64;
use base64::encode;
use chartgeneratorsvg::{chord_list_wasm, StructChart, TraitDraw};
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
pub fn chart(note: &str, fret: u8) -> String {
    let chart: StructChart =
        StructChart::new(note, fret, chartgeneratorsvg::Tuning::C);
    let svg = chart.draw_base();
    let enc = encode(&svg.to_string());
    enc.into()
    //"hello".into()
}

#[wasm_bindgen]
pub fn chord_list() -> String {
    chord_list_wasm().into()
}

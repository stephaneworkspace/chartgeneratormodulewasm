use chartgeneratorsvg::chord::FretID;
use chartgeneratorsvg::interface::{InterfaceWasm, TraitChord, TraitScale};
use std::str::FromStr;
use ukulele_midi::SoundBytes;
use ukulele_midi::Variant;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct UkuleleWasm {}

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
//.into()
}*/
#[wasm_bindgen]
impl UkuleleWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    /// Get a json in Vec<DtoNote> of chord list with multiple svg
    /// More info about the Dto:
    /// use chartgeneratorsvg::generate::{DtoChord, DtoNote};
    ///
    /// # Arguments
    ///
    /// * `note` - Valid note (parsed after in Note::from_str) or empty/invalid
    /// for select all note
    /// * `chord`- Valid chord (parsed after in Chord::from_str) or
    /// 'all'/invalid for select all chord
    /// * `fret_position` - Fret position
    pub fn chord_list(
        &self,
        note: &str,
        chord: &str,
        fret_position: FretID,
    ) -> String {
        InterfaceWasm::chord_list(note, chord, fret_position)
    }

    pub fn chord_list_experimental(
        &self,
        note: &str,
        chord: &str,
        fret_position: FretID,
    ) -> String {
        InterfaceWasm::chord_list_experimental(note, chord, fret_position)
    }
    /*
    #[wasm_bindgen(catch)]
    pub fn path_test(&self, path: &str) -> Result<String, JsValue> {
        return read_file(path: &str);
    }*/

    // TODO Option<&[u8]>
    #[wasm_bindgen(catch)]
    pub fn generate_wav(
        &self,
        variant: &str,
        semitones: &[u8],
        sample_ukulele: Box<[u8]>,
    ) -> Result<Vec<u8>, JsValue> {
        let mut sb: SoundBytes = SoundBytes {
            semitones_midi: semitones,
            midi: &mut Vec::new(),
            wav: &mut Vec::new(),
        };
        match Variant::from_str(variant) {
            Ok(v) => {
                match sb.generate_from_buffer(v, sample_ukulele) {
                    Ok(()) => Ok(sb.get_wav().to_vec()),
                    Err(err) => Err(JsValue::from_str(
                        format!(
                            "Error generate midi->wave I/O in memory: {:?}",
                            err
                        )
                        .as_str(),
                    )), //TODO better
                }
            }
            Err(err) => Err(JsValue::from_str(
                format!("Error generate midi->wave with variation: {:?}", err)
                    .as_str(),
            )),
        }
    }

    pub fn scale_list_select(&self) -> String {
        InterfaceWasm::scale_list_wasm()
    }

    #[wasm_bindgen(catch)]
    pub fn scale_svg_unique(
        &self,
        scale_short: &str,
        tonic: &str,
    ) -> Result<String, JsValue> {
        match InterfaceWasm::scale_print(scale_short, tonic) {
            Ok(res) => Ok(res),
            Err(e) => Err(JsValue::from_str(e.name.as_str())),
        }
    }

    #[wasm_bindgen(catch)]
    pub fn scale_chord_list(
        &self,
        scale_short: &str,
        tonic: &str,
        fret_position: FretID,
    ) -> Result<String, JsValue> {
        match InterfaceWasm::scale_chord_list(scale_short, tonic, fret_position)
        {
            Ok(res) => Ok(res),
            Err(e) => Err(JsValue::from_str(e.name.as_str())),
        }
    }
}

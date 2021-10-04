//! Wasm wrapper functions.
use js_sys::Map as JsMap;
use std::collections::HashMap;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

/// Wasm wrapper for prepare_srt_content().
///
/// Converts Javascript Map into HashMap before feeding to
/// prepare_srt_content().
#[wasm_bindgen]
pub fn wasm_prepare_srt_content(
    text: &str,
    length_in_seconds: u32,
    js_abbr_map: &JsMap,
) -> Result<String, JsValue> {
    let mut abbr_map: HashMap<String, String> = HashMap::new();
    js_abbr_map.for_each(&mut |v, k| {
        if k.is_string() && v.is_string() {
            let ks = k.as_string().unwrap();
            let vs = v.as_string().unwrap();
            abbr_map.insert(ks, vs);
        }
    });

    if abbr_map.len() != js_abbr_map.size().try_into().unwrap() {
        return Err(JsValue::from_str("Invalid abbreviation map."));
    }

    let srt_content = crate::srt::prepare_srt_content(text, length_in_seconds, &abbr_map);

    return Ok(srt_content);
}

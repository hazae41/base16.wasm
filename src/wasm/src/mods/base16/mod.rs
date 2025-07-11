use wasm_bindgen::prelude::*;

use memory_wasm::Memory;

use crate::rjse;

#[wasm_bindgen]
pub fn base16_encode_lower(bytes: &Memory) -> String {
    base16ct::lower::encode_string(&bytes.inner)
}

#[wasm_bindgen]
pub fn base16_encode_upper(bytes: &Memory) -> String {
    base16ct::upper::encode_string(&bytes.inner)
}

#[wasm_bindgen]
pub fn base16_decode_mixed(text: &str) -> Result<Memory, JsError> {
    rjse!(base16ct::mixed::decode_vec(text).map(Memory::new))
}

#[wasm_bindgen]
pub fn base16_decode_lower(text: &str) -> Result<Memory, JsError> {
    rjse!(base16ct::lower::decode_vec(text).map(Memory::new))
}

#[wasm_bindgen]
pub fn base16_decode_upper(text: &str) -> Result<Memory, JsError> {
    rjse!(base16ct::upper::decode_vec(text).map(Memory::new))
}

extern crate web_sys;

use base64::URL_SAFE_NO_PAD;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::str;
use wasm_bindgen::prelude::*;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

#[wasm_bindgen]
pub fn proxy(payload_with_iv: &str) -> String {
    let key = "12345678123456781234567812345678";

    let decoded = base64::decode_config(payload_with_iv, URL_SAFE_NO_PAD).unwrap();

    let iv = String::from_utf8(decoded[..16].to_vec()).unwrap();
    let data = decoded[16..].to_vec();

    let mc = new_magic_crypt!(key, 256, iv);
    let decrypted = mc.decrypt_bytes_to_bytes(&data);
    let result = decrypted.unwrap();
    log!("{}", result.len());
    "".to_string()
}

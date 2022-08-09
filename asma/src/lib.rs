extern crate web_sys;

use aes::Aes256;
use base64::URL_SAFE_NO_PAD;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use md5::compute as md5_digest;
use wasm_bindgen::prelude::*;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

// macro_rules! log {
//     ($($t:tt)*) => {
//         web_sys::console::log_1(&format!($($t)*).into());
//     }
// }

#[wasm_bindgen]
pub fn proxy(payload_with_iv: &str, jwt_token: &str) -> Result<String, JsError> {
    // Improve by using SHA 256 bytes only
    let key = format!("{:x}", md5_digest(jwt_token));

    // Improve by getting rid of the base64 encoding
    let decode_result = base64::decode_config(payload_with_iv, URL_SAFE_NO_PAD);
    if !decode_result.is_ok() {
        return Err(JsError::from(decode_result.unwrap_err()));
    }

    let decoded_payload = decode_result.unwrap();
    if decoded_payload.len() < 16 {
        return Err(JsError::new("Invalid payload length"));
    }

    let iv = decoded_payload[..16].to_vec();
    let data = decoded_payload[16..].to_vec();

    let decrypted = decrypt_aes256(&key.as_bytes(), &iv, &data);
    return Ok(String::from_utf8(decrypted).unwrap());
}

fn decrypt_aes256(key: &[u8], iv: &[u8], data: &[u8]) -> Vec<u8> {
    let mut encrypted_data = data.clone().to_owned();
    let cipher = Aes256Cbc::new_from_slices(&key, &iv).unwrap();
    cipher.decrypt(&mut encrypted_data).unwrap().to_vec()
}

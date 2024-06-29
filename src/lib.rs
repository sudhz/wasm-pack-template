use wasm_bindgen::prelude::*;
use sha2::{Sha256, Digest};
use base64::{encode};
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Work {
    pub result: String,
    pub nonce: i32,
}

#[wasm_bindgen]
pub fn generate_sha256_result(string: &str, maximum_nonce: i32, difficulty_factor: i32, steps: i32, js_fn: &js_sys::Function) -> String {
    let salt = "d@yforcec@ptcha";
    let mut rng = thread_rng();
    let nonce = if maximum_nonce <= 0 {
        rng.gen_range(0..1001)
    } else {
        rng.gen_range(0..maximum_nonce)
    };
    let mut input = format!("{}{}{}", string, salt, nonce);
    for i in 0..difficulty_factor { 
        let hash = Sha256::digest(input.as_bytes());
        let hash_string = encode(&hash);
        input = format!("{}{}{}", hash_string, salt, nonce);
        if i % steps == 0 {
            if let Err(e) = js_fn.call1(&JsValue::null(), &JsValue::from(0)) {
                panic!("Error calling JavaScript function: {:?}", e);
            }
        }
    }
    let result = encode(&Sha256::digest(input.as_bytes()));
    let work = Work {
        nonce,
        result,
    };
    return serde_json::to_string(&work).unwrap();
}
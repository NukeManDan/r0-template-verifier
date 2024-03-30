mod utils;

use bincode::deserialize;
use methods::PROVING_METHOD_ID;
use risc0_zkvm::Receipt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn verify_receipt_buffer(buf: &[u8]) -> String {
    let receipt: Receipt = deserialize(&buf).expect("deserialize receipt");
    receipt.verify(PROVING_METHOD_ID).expect("INVALID RECEIPT!");
    "✅ Receipt is valid".to_string()
}
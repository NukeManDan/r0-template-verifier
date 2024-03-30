#[allow(warnings)]
mod bindings;

use bindings::Guest;
struct Component;
impl Guest for Component {
    fn verify_receipt(raw: Vec<u8>) -> Result<(), String> {
        verify_raw_receipt(raw).map_err(|e| format!("{e}"))
    }
}
bindings::export!(Component with_types_in bindings);

use bincode::deserialize;
use methods::PROVING_METHOD_ID;
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::Receipt;

fn verify_raw_receipt(raw: Vec<u8>) -> Result<(), VerificationError> {
    let receipt: Receipt = deserialize(&raw).expect("deserialize receipt");
    receipt.verify(PROVING_METHOD_ID)
}

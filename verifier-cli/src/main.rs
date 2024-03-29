use bincode::deserialize;
use methods::PROVING_METHOD_ID;
use risc0_zkvm::Receipt;
use risc0_zkp::verify::VerificationError;
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), VerificationError>{
    let path = "testing.receipt";
    let mut file = File::open(path).expect("open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("fill receipt");

    let receipt: Receipt = deserialize(&buf).expect("deserialize receipt");

    let validity = receipt.verify(PROVING_METHOD_ID);

    match validity {
        Ok(_) => {
            println!("✅ Receipt is valid");
            validity
        },
        Err(_) => {
            eprintln!("❌ INVALID Receipt!");
            validity
        },
    }
}
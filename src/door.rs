use base64::{Engine as _, engine::{general_purpose}};
use libbruteforce::{BasicCrackParameter, CrackParameter, TargetHashInput};
use libbruteforce::hash_fncs::sha256_hashing;
use libbruteforce::symbols;

fn decode_bytes(bytes: &Vec<u8>) -> Result<String, String> {
    if let Ok(hash) = std::str::from_utf8(&bytes) {
        let sha256_hash = "3d7edde33628331676b39e19a3f2bdb3c583960ad8d865351a32e2ace7d8e02d";
        let max_len = 3;
        let min_len = 0;
        let alphabet = symbols::Builder::new().with_digits().build();

        let res = CrackParameter::new(
            BasicCrackParameter::new(alphabet, max_len, min_len, false),
            sha256_hashing(TargetHashInput::HashAsStr(hash)),
        );
    }

    return Err(String::from("Failed to read hash as string"));
}

pub fn decode(message: &str) -> Result<String, String> {
    if let Ok(bytes) = general_purpose::STANDARD.decode(message) {
        let result = decode_bytes(&bytes);

        if let Ok(message) = &result {
            println!("Message: {}", message);
        }

        return result;
    }

    return Err(String::from("Failed to parse Base64"));
}

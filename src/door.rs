use base64::{Engine as _, engine::{general_purpose}};

fn bruteforce(hash: &str, dictionary: &Vec<String>) -> Option<String> {
    for word in dictionary {
        let digest = md5::compute(word);
        if format!("{:x}", digest) == hash {
            return Some(word.clone());
        }
    }

    return None;
}

fn decode_bytes(bytes: &Vec<u8>, dictionary: &Vec<String>) -> Result<String, String> {
    if let Ok(hash) = std::str::from_utf8(&bytes) {
        return bruteforce(hash, dictionary).ok_or(String::from("No results"));
    }

    return Err(String::from("Failed to read hash as string"));
}

pub fn decode(message: &str, dictionary: &Vec<String>) -> Result<String, String> {
    if let Ok(bytes) = general_purpose::STANDARD.decode(message) {
        let result = decode_bytes(&bytes, dictionary);

        if let Ok(message) = &result {
            println!("Message: {}", message);
        }

        return result;
    }

    return Err(String::from("Failed to parse Base64"));
}

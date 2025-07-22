use crypto::{aes128_decrypt, AesError};
use serialize::from_base64_file;

pub fn run() -> bool {
    let input = match from_base64_file("data/set01/challenge07.txt") {
        Ok(content) => content,
        Err(_) => return false,
    };
    static KEY: &[u8] = b"YELLOW SUBMARINE";

    match solve(&input, KEY) {
        Ok(result) => {
            let plaintext = String::from_utf8_lossy(&result);
            dbg!(plaintext);
            true
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        },
    }
}

fn solve(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError> {
    let plaintext = aes128_decrypt(&ciphertext, key, crypto::AesMode::ECB)?;
    Ok(plaintext)
}
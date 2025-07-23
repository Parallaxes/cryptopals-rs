use crypto::{aes128_decrypt, AesError};
use serialize::from_base64_file;

pub fn run() -> bool {
    let input = match from_base64_file("data/set01/challenge07.txt") {
        Ok(content) => content,
        Err(_) => return false,
    };
    static KEY: &[u8] = b"YELLOW SUBMARINE";
    static EXPECTED: &str = "I'm back and I'm ringin' the bell \n";

    match solve(&input, KEY) {
        Ok(result) => {
            let plaintext = String::from_utf8_lossy(&result);
            let first_line = plaintext.lines().next().unwrap_or("").to_string() + "\n";
            dbg!(plaintext);
            first_line == EXPECTED
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
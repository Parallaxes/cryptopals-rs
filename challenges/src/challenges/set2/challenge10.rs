use serialize::from_base64_file;
use crypto::{aes128_decrypt, AesError, AesMode};

pub fn run() -> bool {
    let input = from_base64_file("data/set02/challenge10.txt")  
        .expect("Failed to read file");
    static KEY: &[u8] = b"YELLOW SUBMARINE";
    static IV: &[u8; 16] = &[0u8; 16];
    static EXPECTED: &str = "I'm back and I'm ringin' the bell \n";

    match solve(&input, KEY, IV) {
        Ok(result) => {
            let plaintext = String::from_utf8_lossy(&result);
            let first_line = plaintext.lines().next().unwrap_or("").to_string() + "\n";
            dbg!(plaintext);
            first_line == EXPECTED
        },
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}

fn solve(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, AesError> {
    let iv_array: [u8; 16] = iv.try_into()
        .map_err(|_| AesError::InvalidBlockSize)?;
    aes128_decrypt(ciphertext, key, AesMode::CBC { iv: iv_array })
}
use std::{collections::HashSet, fs};

use serialize::{to_hex, from_hex};

pub fn run() -> bool {
    let input = match fs::read_to_string("data/set01/challenge08.txt") {
        Ok(content) => content,
        Err(_) => return false,
    };
    static EXPECTED: &str = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a";

    match solve(input) {
        Ok(result) => {
            let hex = &to_hex(&result);
            dbg!(hex);
            hex == EXPECTED
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}

fn solve(hex_input: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    for line in hex_input.lines() {
        let ciphertext = from_hex(line)?;

        if has_duplicate(&ciphertext) {
            return Ok(ciphertext);
        }
    }

    Err("Failed to find ECB encoded ciphertext".into())
}

fn has_duplicate(ciphertext: &[u8]) -> bool {
    const BLOCK_SIZE: usize = 16;

    if ciphertext.len() % BLOCK_SIZE != 0 {
        return false;
    }

    let mut seen_blocks = HashSet::new();

    for chunk in ciphertext.chunks(BLOCK_SIZE) {
        if !seen_blocks.insert(chunk) {
            return true;
        }
    }

    false
}
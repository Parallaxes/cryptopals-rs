use crypto::{aes::{block, pkcs7_pad, AES128_BLOCK_SIZE}, aes128_encrypt};
use serialize::from_base64;

use std::collections::{HashSet, HashMap};

const UNKNOWN_KEY: [u8; 16] = [61, 116, 8, 190, 121, 122, 26, 216, 236, 11, 109, 122, 38, 85, 180, 163];

struct Oracle12 {
    key: [u8; 16],
    unknown_suffix: Vec<u8>,
}

impl Oracle12 {
    fn new(unknown_suffix: Vec<u8>) -> Self {
        Self {
            key: UNKNOWN_KEY,
            unknown_suffix,
        }
    }

    fn encrypt(&self, attacker_input: &[u8]) -> Vec<u8> {
        let mut plaintext = Vec::new();
        plaintext.extend_from_slice(attacker_input);
        plaintext.extend_from_slice(&self.unknown_suffix);

        let padded = pkcs7_pad(&plaintext, AES128_BLOCK_SIZE);
        aes128_encrypt(&padded, &self.key, crypto::AesMode::ECB).unwrap()
    }
}


pub fn run() -> bool {
    let unknown_str: Vec<u8> = from_base64("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\naGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\ndXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\nYnkK")
        .expect("Failed to decode base64");
    static EXPECTED: &str = "Rollin' in my 5.0\nWith my rag-top down so my hair can blow\nThe girlies on standby waving just to say hi\nDid you stop? No, I just drove by\n";

    match solve(&unknown_str) {
        Ok(result) => {
            let plaintext = String::from_utf8_lossy(&result);
            dbg!(&plaintext);
            plaintext == EXPECTED
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}

fn solve(unknown_str: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let oracle = Oracle12::new(unknown_str.to_vec());
    let block_size = find_block_size(&oracle);

    if !detect_ecb(&oracle) {
        return Err("Oracle is not using ECB mode".into());
    }

    let mut discovered = Vec::new();

    // Discover bytes
    while discovered.len() < unknown_str.len() {
        match crack_next_byte(&oracle, &discovered, block_size) {
            Some(byte) => discovered.push(byte),
            None => break, // Probably hit padding
        }
    }

    Ok(discovered)
}

fn find_block_size(oracle: &Oracle12) -> usize {
    let init_len = oracle.encrypt(b"").len();

    for i in 1..=64 {
        let test_input = vec![b'A'; i];
        let new_len = oracle.encrypt(&test_input).len();

        if new_len > init_len {
            return new_len - init_len;
        }
    }

    panic!("Could not find block size within 64 bytes");
}

fn detect_ecb(oracle: &Oracle12) -> bool {
    let plaintext = b"A".repeat(48);
    let ciphertext = oracle.encrypt(&plaintext);
    let mut seen_blocks = HashSet::new();

    // ECB retains patterns in plaintext
    for chunk in ciphertext.chunks_exact(AES128_BLOCK_SIZE) {
        if !seen_blocks.insert(chunk) {
            return true;
        }
    }

    false
}

fn build_dictionary(oracle: &Oracle12, known_prefix: &[u8], block_size: usize) -> HashMap<Vec<u8>, u8> {
    let mut dictionary = HashMap::new();
    let current_block_index = known_prefix.len() / block_size;

    // Create padding to make test block 1 byte short
    let padding_len = block_size - 1 - (known_prefix.len() % block_size);
    let mut test_input = vec![b'A'; padding_len];
    test_input.extend_from_slice(known_prefix);

    // Try possible bytes from 0 -> 255
    for byte in 0..=255 {
        let mut full_input = test_input.clone();
        full_input.push(byte);

        let ciphertext = oracle.encrypt(&full_input);
        let block_start = current_block_index * block_size;
        let block_end = block_start + block_size;
        
        if ciphertext.len() >= block_end {
            let target_block = ciphertext[block_start..block_end].to_vec();
            dictionary.insert(target_block, byte);
        }
    }

    dictionary
}

fn crack_next_byte(oracle: &Oracle12, known_bytes: &[u8], block_size: usize) -> Option<u8> {
    // Calculate which block we're currently attacking
    let current_block_index = known_bytes.len() / block_size;
    
    // Create dictionary for this position
    let dictionary = build_dictionary(oracle, known_bytes, block_size);

    // Create 1 byte short input
    let padding_len = block_size - 1 - (known_bytes.len() % block_size);
    let short_input = vec![b'A'; padding_len];

    // Get the target ciphertext block (the block we're currently attacking)
    let target_ciphertext = oracle.encrypt(&short_input);
    let block_start = current_block_index * block_size;
    let block_end = block_start + block_size;
    
    if target_ciphertext.len() < block_end {
        return None; // No more blocks to crack
    }
    
    let target_block = target_ciphertext[block_start..block_end].to_vec();

    // Look up in the dictionary
    dictionary.get(&target_block).copied()
}
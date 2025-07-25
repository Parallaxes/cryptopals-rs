use crypto::utils::oracle::*;
use crypto::aes::{aes128_encrypt, AesMode, pkcs7_pad};
use rand::{rng, Rng};
use crypto::aes::AES128_BLOCK_SIZE;

use std::collections::HashSet;

pub fn run() -> bool {
    match solve() {
        Ok(result) => {
            dbg!(result);
            result == 1.0
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum DetectedMode {
    ECB,
    CBC,
}

struct Oracle11 {
    key: [u8; 16],
    mode: DetectedMode,
    iv: Option<[u8; 16]>,
}

impl Oracle11 {
    fn new() -> Self {
        let mut rng = rng();
        let mode = if rng.random_bool(0.5) { DetectedMode::ECB } else { DetectedMode::CBC };
        let iv = if matches!(mode, DetectedMode::CBC) { Some(random_iv()) } else { None };

        Self {
            key: random_aes_key(),
            mode,
            iv,
        }
    }

    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut padded = Vec::new();
        padded.extend_from_slice(&random_padding(5, 10));
        padded.extend_from_slice(plaintext);
        padded.extend_from_slice(&random_padding(5, 10));

        let padded = pkcs7_pad(&padded, AES128_BLOCK_SIZE);

        let aes_mode = match self.mode {
            DetectedMode::ECB => AesMode::ECB,
            DetectedMode::CBC => AesMode::CBC { iv: self.iv.unwrap() },
        };

        aes128_encrypt(&padded, &self.key, aes_mode).unwrap()
    }

    fn get_actual_mode(&self) -> &DetectedMode {
        &self.mode
    }
}

fn solve() -> Result<f64, Box<dyn std::error::Error>> {
    let plaintext = b"A".repeat(48);
    let mut correct = 0;
    let trials = 1000;

    for _ in 0..trials {
        let oracle = Oracle11::new();
        let ciphertext = oracle.encrypt(&plaintext);

        let detected_mode = detect(&ciphertext);

        if detected_mode == *oracle.get_actual_mode() {
            correct += 1;
        }
    }

    Ok(correct as f64 / trials as f64)
}

fn detect(ciphertext: &[u8]) -> DetectedMode {
    let mut seen_blocks = HashSet::new();

    // ECB retains patterns in plaintext
    for chunk in ciphertext.chunks_exact(AES128_BLOCK_SIZE) {
        if !seen_blocks.insert(chunk) {
            return DetectedMode::ECB;
        }
    }

    DetectedMode::CBC
}
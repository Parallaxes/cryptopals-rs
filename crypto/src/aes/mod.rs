// crypto/src/aes/mod.rs
pub mod block;
pub mod ecb;
pub mod cbc;
pub mod padding;

use crate::common::AesError;

pub use padding::{pkcs7_pad, pkcs7_unpad};

pub const AES128_BLOCK_SIZE: usize = 16;

#[derive(Debug, Clone)]
pub enum AesMode {
    ECB,
    CBC { iv: [u8; AES128_BLOCK_SIZE] },
}

/// Main public AES-128 encrypt API
pub fn aes128_encrypt(plaintext: &[u8], key: &[u8], mode: AesMode) -> Result<Vec<u8>, AesError> {
    match mode {
        AesMode::ECB => ecb::encrypt(plaintext, key),
        AesMode::CBC { iv } => cbc::encrypt(plaintext, key, &iv),
    }
}

/// Main public AES-128 decrypt API
pub fn aes128_decrypt(ciphertext: &[u8], key: &[u8], mode: AesMode) -> Result<Vec<u8>, AesError> {
    match mode {
        AesMode::ECB => ecb::decrypt(ciphertext, key),
        AesMode::CBC { iv } => cbc::decrypt(ciphertext, key, &iv),
    }
}
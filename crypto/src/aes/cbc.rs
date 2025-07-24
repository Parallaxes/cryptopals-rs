// crypto/src/aes/cbc.rs
use crate::common::AesError;
use super::block::{encrypt_block, decrypt_block};
use xor::fixed_xor;

const AES128_BLOCK_SIZE: usize = 16;

pub fn encrypt(plaintext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, AesError> {
    if plaintext.len() % AES128_BLOCK_SIZE != 0 {
        return Err(AesError::InvalidBlockSize);
    }

    let mut result = Vec::with_capacity(plaintext.len());
    let mut previous_block = iv.to_vec();

    for block in plaintext.chunks_exact(AES128_BLOCK_SIZE) {
        let xor_block = fixed_xor(block, &previous_block)
            .map_err(|_| AesError::InvalidBlockSize)?;

        let ciphertext_block = encrypt_block(&xor_block, key)?;

        result.extend_from_slice(&ciphertext_block);
        previous_block = ciphertext_block;
    }

    Ok(result)
}

pub fn decrypt(ciphertext: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, AesError> {
    if ciphertext.len() % AES128_BLOCK_SIZE != 0 {
        return Err(AesError::InvalidBlockSize);
    }

    let mut result = Vec::with_capacity(ciphertext.len());
    let mut previous_block = iv;

    for block in ciphertext.chunks_exact(AES128_BLOCK_SIZE) {
        let decrypted_block = decrypt_block(block, key)?;
        let plaintext_block = fixed_xor(&decrypted_block, previous_block)
            .map_err(|_| AesError::InvalidBlockSize)?;

        result.extend_from_slice(&plaintext_block);
        previous_block = block;
    }

    Ok(result)
}
use crate::common::AesError;
use super::block::{encrypt_block, decrypt_block};

const AES128_BLOCK_SIZE: usize = 16;

pub fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError> {
    if plaintext.len() % AES128_BLOCK_SIZE != 0 {
        return Err(AesError::InvalidBlockSize);
    }

    let mut ciphertext = Vec::with_capacity(plaintext.len());
    
    for chunk in plaintext.chunks_exact(AES128_BLOCK_SIZE) {
        let encrypted_block = encrypt_block(chunk, key)?;
        ciphertext.extend_from_slice(&encrypted_block);
    }

    Ok(ciphertext)
}

pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError> {
    if ciphertext.len() % AES128_BLOCK_SIZE != 0 {
        return Err(AesError::InvalidBlockSize);
    }

    let mut plaintext = Vec::with_capacity(ciphertext.len());
    
    for chunk in ciphertext.chunks_exact(AES128_BLOCK_SIZE) {
        let decrypted_block = decrypt_block(chunk, key)?;
        plaintext.extend_from_slice(&decrypted_block);
    }

    Ok(plaintext)
}
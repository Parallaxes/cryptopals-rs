use aes::{Aes128, cipher::{BlockDecrypt, BlockEncrypt, KeyInit}};
use aes::cipher::generic_array::GenericArray;

const AES128_BLOCK_SIZE: usize = 16; // 128 bits

#[derive(Debug)]
pub enum AesMode {
    ECB,
    CBC {iv : [u8; AES128_BLOCK_SIZE] },
}

#[derive(Debug)]
pub enum AesError {
    InvalidKeyLength,
    InvalidBlockSize,
    InvalidPadding,
}

/// Decrypt a single AES-128 block (16 bytes) using the raw cipher
fn decrypt_block_raw(block: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError> {
    if key.len() != 16 {
        return Err(AesError::InvalidKeyLength);
    }
    if block.len() != AES128_BLOCK_SIZE {
        return Err(AesError::InvalidBlockSize);
    }

    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut block_copy = *GenericArray::from_slice(block);
    cipher.decrypt_block(&mut block_copy);

    Ok(block_copy.to_vec())
}

/// Encrypt a single AES-128 block (16 bytes) using the raw cipher
fn encrypt_block_raw(block: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError> {
    if key.len() != 16 {
        return Err(AesError::InvalidKeyLength);
    }
    if block.len() != AES128_BLOCK_SIZE {
        return Err(AesError::InvalidBlockSize);
    }
    
    let cipher = Aes128::new(GenericArray::from_slice(key));
    let mut block_copy = *GenericArray::from_slice(block);
    cipher.encrypt_block(&mut block_copy);

    Ok(block_copy.to_vec())
}

/// Main AES-128 encryption function with mode selection
pub fn aes128_decrypt(ciphertext: &[u8], key: &[u8], mode: AesMode) -> Result<Vec<u8>, AesError> {
    match mode {
        AesMode::ECB => decrypt_ecb(ciphertext, key),
        AesMode::CBC { iv } => decrypt_cbc(ciphertext, key, &iv),
    }
}

/// Main AES-128 encryption function with mode selection
pub fn aes128_encrypt(plaintext: &[u8], key: &[u8], mode: AesMode) -> Result<Vec<u8>, AesError> {
    match mode {
        AesMode::ECB => encrypt_ecb(plaintext, key),
        AesMode::CBC { iv } => encrypt_cbc(plaintext, key, &iv),
    }
}

fn decrypt_ecb(ciphertext: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError> {
    // TODO: handle padding
    if ciphertext.len() % AES128_BLOCK_SIZE != 0 {
        return Err(AesError::InvalidBlockSize);
    }

    let mut plaintext = Vec::new();
    for chunk in ciphertext.chunks_exact(AES128_BLOCK_SIZE) {
        let decrypted_block = decrypt_block_raw(chunk, key)?;
        plaintext.extend_from_slice(&decrypted_block);
    }

    Ok(plaintext)
}

fn encrypt_ecb(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError> {
    // TODO: handle padding
    let mut ciphertext = Vec::new();
    for chunk in plaintext.chunks_exact(AES128_BLOCK_SIZE) {
        let encrypted_block = encrypt_block_raw(chunk, key)?;
        ciphertext.extend_from_slice(&encrypted_block);
    }

    Ok(ciphertext)
}

fn decrypt_cbc(ciphertext: &[u8], iv: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError>{
    unimplemented!()
}

fn encrypt_cbc(plaintext: &[u8], iv: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError>{
    unimplemented!()
}

pub fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_len = data.len() % block_size;
    let padding_req = if padding_len == 0 {
        block_size
    } else {
        block_size - padding_len
    };

    let mut padded = Vec::with_capacity(data.len() + padding_req);
    padded.extend_from_slice(data);

    for _ in 0..padding_req {
        padded.push(padding_req as u8);
    }

    padded
}

pub fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, AesError> {
    if data.is_empty() {
        return Err(AesError::InvalidPadding);
    }

    let padding_len = *data.last().unwrap() as usize;

    if padding_len == 0 || padding_len > data.len() {
        return Err(AesError::InvalidPadding);
    }

    let padding_start = data.len() - padding_len;
    for &byte in &data[padding_start..] {
        if byte != padding_len as u8 {
            return Err(AesError::InvalidPadding);
        }
    }

    Ok(data[..padding_start].to_vec())
}
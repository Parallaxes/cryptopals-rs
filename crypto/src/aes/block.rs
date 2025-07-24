use aes::{Aes128, cipher::{BlockDecrypt, BlockEncrypt, KeyInit}};
use aes::cipher::generic_array::GenericArray;
use crate::common::AesError;

const AES128_BLOCK_SIZE: usize = 16;

pub fn encrypt_block(block: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError> {
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

pub fn decrypt_block(block: &[u8], key: &[u8]) -> Result<Vec<u8>, AesError> {
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
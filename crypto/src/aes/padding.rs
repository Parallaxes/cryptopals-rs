use super::AesError;

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
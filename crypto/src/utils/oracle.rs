use rand::{rng, Rng};

pub fn random_aes_key() -> [u8; 16] {
    let mut rng = rng();
    let mut key = [0u8; 16];

    for byte in key.iter_mut() {
        *byte = rng.random();
    }

    key
}

pub fn random_iv() -> [u8; 16] {
    let mut rng = rng();
    let mut iv = [0u8; 16];

    for byte in iv.iter_mut() {
        *byte = rng.random();
    }

    iv
}

pub fn random_bytes(len: usize) -> Vec<u8> {
    let mut rng = rng();
    let mut bytes = Vec::with_capacity(len);

    for byte in bytes.iter_mut() {
        *byte = rng.random();
    }

    bytes
}

pub fn random_padding(min: usize, max: usize) -> Vec<u8> {
    let mut rng = rng();
    let len = rng.random_range(min..=max);
    random_bytes(len)
}
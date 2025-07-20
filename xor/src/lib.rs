#[derive(Debug, Clone, PartialEq)]
pub enum XorError {
    LengthMismatch { left: usize, right: usize },
}

impl std::fmt::Display for XorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XorError::LengthMismatch { left, right } => {
                write!(f, "Length mismatch: {} vs {}", left, right)
            }
        }
    }
}

impl std::error::Error for XorError {}

pub fn fixed_xor(a: &[u8], b: &[u8]) -> Result<Vec<u8>, XorError> {
    if a.len() != b.len() {
        return Err(XorError::LengthMismatch { left: a.len(), right: b.len() });
    }

    Ok(a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect())
}

pub fn single_byte_xor(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|b| b ^ key).collect()
}
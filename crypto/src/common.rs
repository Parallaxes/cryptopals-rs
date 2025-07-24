#[derive(Debug, Clone)]
pub enum AesError {
    InvalidKeyLength,
    InvalidBlockSize,
    InvalidPadding,
}
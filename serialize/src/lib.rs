pub trait Serialize {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializeError>
    where Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]
pub enum SerializeError {
    InvalidLength,
    InvalidFormat,
    InvalidData(String),
}

impl std::fmt::Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializeError::InvalidLength => write!(f, "Invalid data length"),
            SerializeError::InvalidFormat => write!(f, "Invalid data format"),
            SerializeError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl std::error::Error for SerializeError {}

pub fn to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

pub fn from_hex(hex: &str) -> Result<Vec<u8>, SerializeError> {
    if hex.len() % 2 != 0 {
        return Err(SerializeError::InvalidLength);
    }

    hex.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| {
            let hex_pair: String = chunk.iter().collect();
            u8::from_str_radix(&hex_pair, 16)
                .map_err(|_| SerializeError::InvalidFormat)
        })
        .collect()
}
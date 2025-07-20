const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub trait Serialize {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, SerializeError>
        where Self: Sized;
}

pub trait FromHex {
    fn from_hex(&self) -> Vec<u8>;
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

pub fn to_base64(bytes: &[u8]) -> String {
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        match chunk.len() {
            3 => {
                let b1 = chunk[0] as u32;
                let b2 = chunk[1] as u32;
                let b3 = chunk[2] as u32;

                let combined = (b1 << 16) | (b2 << 8) | b3;

                let idx1 = ((combined >> 18) & 0x3F) as usize;
                let idx2 = ((combined >> 12) & 0x3F) as usize;
                let idx3 = ((combined >> 6) & 0x3F) as usize;
                let idx4 = (combined & 0x3F) as usize;

                result.push(BASE64_CHARS[idx1] as char);
                result.push(BASE64_CHARS[idx2] as char);
                result.push(BASE64_CHARS[idx3] as char);
                result.push(BASE64_CHARS[idx4] as char);
            }
            2 => {
                let b1 = chunk[0] as u32;
                let b2 = chunk[1] as u32;

                let combined = (b1 << 16) | (b2 << 8);

                let idx1 = ((combined >> 18) & 0x3F) as usize;
                let idx2 = ((combined >> 12) & 0x3F) as usize; 
                let idx3 = ((combined >> 6) & 0x3F) as usize;

                result.push(BASE64_CHARS[idx1] as char);
                result.push(BASE64_CHARS[idx2] as char);
                result.push(BASE64_CHARS[idx3] as char);
                result.push('=');
            }
            1 => {
                let b1 = chunk[0] as u32;

                let combined = b1 << 16;

                let idx1 = ((combined >> 18) & 0x3F) as usize;
                let idx2 = ((combined >> 12) & 0x3F) as usize;

                result.push(BASE64_CHARS[idx1] as char);
                result.push(BASE64_CHARS[idx2] as char);
                result.push('=');
                result.push('=');
            }
            _ => {
                unreachable!();
            }
        }
    }

    result
}

pub fn from_base64(base64: &str) -> Result<Vec<u8>, SerializeError> {
    todo!("Implement base64 decoding from scratch")
}
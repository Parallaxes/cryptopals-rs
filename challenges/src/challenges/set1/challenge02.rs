use serialize::{from_hex, to_hex};
use xor::fixed_xor;

pub fn run() -> bool {
    static INPUT: &str = "1c0111001f010100061a024b53535009181c";
    static TARGET: &str = "686974207468652062756c6c277320657965";
    static EXPECTED: &str = "746865206b696420646f6e277420706c6179";

    match solve(INPUT, TARGET) {
        Ok(result) => result == EXPECTED,
        Err(_) => false, 
    }
}

fn solve(a: &str, b: &str) -> Result<String, Box<dyn std::error::Error>> {
    let bytes_a = from_hex(a)?;
    let bytes_b = from_hex(b)?;
    let xor_result = fixed_xor(&bytes_a, &bytes_b)?;
    Ok(to_hex(&xor_result))
}
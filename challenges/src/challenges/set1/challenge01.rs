use serialize::{from_hex, to_base64};

pub fn run() -> bool {
    static INPUT: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    static EXPECTED: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    match solve(INPUT) {
        Ok(result) => result == EXPECTED,
        Err(_) => false,
    }
}

fn solve(hex_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = from_hex(hex_input)?;
    Ok(to_base64(&bytes))
}
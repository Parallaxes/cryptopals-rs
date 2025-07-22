use serialize::to_hex;
use xor::repeating_key_xor;

pub fn run() -> bool {
    static INPUT_TEXT: &str = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    static INPUT_KEY: &str = "ICE";
    static EXPECTED: &str = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    match solve(INPUT_TEXT, INPUT_KEY) {
        Ok(result) => {
            let result_hex = to_hex(&result);
            dbg!(&result_hex);
            result_hex == EXPECTED
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        },
    }
}

fn solve(plaintext_input: &str, plaintext_key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(repeating_key_xor(plaintext_input.as_bytes(), plaintext_key.as_bytes()))
}
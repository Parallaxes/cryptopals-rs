use crypto::aes::padding::pkcs7_pad;

pub fn run() -> bool {
    static INPUT: &str = "YELLOW SUBMARINE";
    static BLOCK_SIZE: usize = 20;
    static EXPECTED: &[u8] = &[89, 69, 76, 76, 79, 87, 32, 83, 85, 66, 77, 65, 82, 73, 78, 69, 4, 4, 4, 4,];

    match solve(INPUT.as_bytes(), BLOCK_SIZE) {
        Ok(result) => {
            dbg!(&result);
            result == EXPECTED
        }
        Err(_) => false,
    }
}

fn solve(input: &[u8], block_size: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    Ok(pkcs7_pad(input, block_size))
}
use serialize::from_hex;
use xor::single_byte_xor;

// Using relative frequencies of letters in general english plaintext from Lewand (2000)
// Source: R. Lewand, Cryptological Mathematics. 2000.
// DOI: https://doi.org/10.1090/clrm/016.
const ENGLISH_FREQUENCIES: &[(char, f32)] = &[
    ('a', 8.167), ('b', 1.492), ('c', 2.782), ('d', 4.253), ('e', 12.702),
    ('f', 2.228), ('g', 2.015), ('h', 6.094), ('i', 6.966), ('j', 0.153),
    ('k', 0.772), ('l', 4.025), ('m', 2.406), ('n', 6.749), ('o', 7.507),
    ('p', 1.929), ('q', 0.095), ('r', 5.987), ('s', 6.327), ('t', 9.056),
    ('u', 2.758), ('v', 0.978), ('w', 2.360), ('x', 0.150), ('y', 1.974),
    ('z', 0.074),
];

pub fn run() -> bool {
    static INPUT: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    static EXPECTED: char = 'X';

    match solve(INPUT) {
        Ok(result) => {
            let bytes = from_hex(INPUT).unwrap();
            dbg!(String::from_utf8_lossy(&single_byte_xor(&bytes, result)));
            result as char == EXPECTED
        },
        Err(_) => false, 
    }
}

fn solve(hex_input: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let mut scores: Vec<(u8, f32)> = Vec::new();
    let bytes = from_hex(hex_input)?;

    for key in 0..=255 {
        let buffer: Vec<u8> = single_byte_xor(&bytes, key);
        scores.push((key, score(&buffer)));
    }

    let (best_key, _) = scores.iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
    Ok(*best_key)
}

pub fn score(bytes: &[u8]) -> f32 {
    let mut score = 0.0;
    let mut printable_cnt = 0;

    for &byte in bytes {
        match byte {
            // ASCII chars -> use freq analysis
            b'a'..=b'z' | b'A'..b'Z' => {
                printable_cnt += 1;
                let c = (byte | 0x20) as char;
                score += ENGLISH_FREQUENCIES.iter()
                    .find(|(ch, _)| *ch == c)
                    .map(|(_, freq)| *freq)
                    .unwrap_or(0.5);
            }
            // Common punctuation and whitespace
            b' ' => { score += 13.0; printable_cnt += 1; }
            b'.' | b',' | b'!' | b'?' | b';' | b':' | b'\'' | b'"' => {
                score += 0.5; 
                printable_cnt += 1;
            }
            // Other printable ASCII
            0x20..=0x7E => {
                score += 0.1;
                printable_cnt += 1;
            }
            // Not printable -> penalty
            _ => score -= 1.0,
        }
    }

    // Heavily weight ratio of printable chars
    let printable_ratio = printable_cnt as f32 / bytes.len() as f32;
    score * printable_ratio * printable_ratio
}
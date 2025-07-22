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
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        },
    }
}

fn solve(hex_input: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let mut scores: Vec<(u8, f32)> = Vec::new();
    let bytes = from_hex(hex_input)?; // APC, input is a hex str

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
    let mut printable_count = 0;
    let mut letter_count = 0;

    for &byte in bytes {
        match byte {
            // Letters - use frequency analysis
            b'a'..=b'z' | b'A'..=b'Z' => {
                printable_count += 1;
                letter_count += 1;
                let c = (byte | 0x20) as char; // Convert to lowercase
                score += ENGLISH_FREQUENCIES.iter()
                    .find(|(ch, _)| *ch == c)
                    .map(|(_, freq)| *freq)
                    .unwrap_or(0.5);
            }
            // Space is very common in English
            b' ' => { 
                score += 12.0; 
                printable_count += 1; 
            }
            // Common punctuation
            b'.' | b',' | b'!' | b'?' | b';' | b':' | b'\'' | b'"' | b'\n' | b'\r' => {
                score += 0.5; 
                printable_count += 1;
            }
            // Other printable ASCII
            0x20..=0x7E => {
                score += 0.1;
                printable_count += 1;
            }
            // Non-printable - smaller penalty
            _ => score -= 10.0,
        }
    }

    // Bonus for high ratio of printable characters
    let printable_ratio = printable_count as f32 / bytes.len() as f32;
    let letter_ratio = letter_count as f32 / bytes.len() as f32;
    
    // Combine the scores
    score * printable_ratio + letter_ratio * 10.0
}
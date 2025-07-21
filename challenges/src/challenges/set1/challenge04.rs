use serialize::from_hex;
use xor::single_byte_xor;
use super::challenge03::score;

#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub key: u8,
    pub plaintext: Vec<u8>,
    pub score: f32,
    pub line_number: usize,
}

pub fn run() -> bool {
    let input = std::fs::read_to_string("data/set01/challenge04.txt")
        .expect("Failed to read input file");
    static EXPECTED: &str = "Now that the party is jumping\n";

    match solve(&input) {
        Some(result) => {
            dbg!(result.line_number, result.key, String::from_utf8_lossy(&result.plaintext));
            String::from_utf8_lossy(&result.plaintext) == EXPECTED
        }
        None => false,
    }
}

fn solve(file_input: &str) -> Option<DetectionResult> {
    let mut best_result: Option<DetectionResult> = None;

    for (line_number, line) in file_input.lines().enumerate() {
        let bytes = match from_hex(line) {
            Ok(b) => b,
            Err(_) => continue, // Skip malformed lines
        };

        // Find best key for this line
        let line_result = (0u8..=255)
            .map(|key| {
                let decrypted = single_byte_xor(&bytes, key);
                let line_score = score(&decrypted);
                DetectionResult {
                    key,
                    plaintext: decrypted,
                    score: line_score,
                    line_number,
                }
            })
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal));

        // Update global best if this line is better
        if let Some(line_best) = line_result {
            match &best_result {
                None => best_result = Some(line_best),
                Some(current_best) if line_best.score > current_best.score => {
                    best_result = Some(line_best);
                }
                _ => {} // Current best is still better
            }
        }
    }

    best_result
}
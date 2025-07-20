use serialize::from_hex;
use xor::single_byte_xor;
use super::challenge03::score;

pub fn run() -> bool {
    let input = std::fs::read_to_string("data/set01/challenge04.txt")
        .expect("Failed to read input file");

    match solve(input) {
        Ok(result) => {
            dbg!(result.0, result.1);
            true
        }
        Err(_) => false
    }
}

fn solve(file_input: String) -> Result<(u8, Vec<u8>), Box<dyn std::error::Error>> {
    let mut scores: Vec<(u8, f32, Vec<u8>)> = Vec::new();

    for line in file_input.lines() {
        let mut local_scores: Vec<(u8, f32, Vec<u8>)> = Vec::new();
        let bytes = from_hex(line)?;

        for key in 0..=255 {
            let buffer = single_byte_xor(&bytes, key);
            local_scores.push((key, score(&buffer), buffer));
        }

        let (best_local_key, best_local_score, best_local_buffer) = local_scores.iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        scores.push((*best_local_key, *best_local_score, best_local_buffer.clone()));
    }

    let (best_key, _, best_buffer) = scores.iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    Ok((*best_key, best_buffer.clone()))
}
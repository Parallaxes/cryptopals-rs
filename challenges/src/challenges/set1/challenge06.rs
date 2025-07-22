use serialize::from_base64_file;
use xor::{repeating_key_xor, single_byte_xor};
use super::challenge03::score;

pub fn run() -> bool {
    let input = match from_base64_file("data/set01/challenge06.txt") {
        Ok(content) => content,
        Err(_) => return false,
    };
    static EXPECTED: &str = "Terminator X: Bring the noise";

    match solve(&input) {
        Ok(result) => {
            let decrypted = repeating_key_xor(&input, &result);
            let key = String::from_utf8_lossy(&result);

            dbg!(&key);
            dbg!(String::from_utf8_lossy(&decrypted));

            key == EXPECTED
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            false
        },
    }
}

fn solve(ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let candidates = find_best_keysizes(ciphertext, 3);

    // Try each keysize and find best size overall
    candidates
        .into_iter()
        .map(|keysize| break_repeating_xor(&ciphertext, keysize))
        .max_by(|a, b| {
            let score_a = score(&repeating_key_xor(&ciphertext, &a));
            let score_b = score(&repeating_key_xor(&ciphertext, &b));
            score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .ok_or_else(|| "No valid key found".into())
}

fn break_repeating_xor(ciphertext: &[u8], keysize: usize) -> Vec<u8> {
    // Transpose group bytes by their pos keys
    let mut key_positions = vec![Vec::new(); keysize];

    for (byte_index, &byte) in ciphertext.iter().enumerate() {
        key_positions[byte_index % keysize].push(byte);
    }

    // Find the best key for each positions
    key_positions
        .iter()
        .map(|block| find_best_single_byte_key(block))
        .collect()
}

fn find_best_single_byte_key(block: &[u8]) -> u8 {
    (0u8..=255)
        .map(|key| {
            let decrypted = single_byte_xor(block, key);
            (key, score(&decrypted))
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(key, _)| key)
        .unwrap_or(0)
}

fn find_best_keysizes(ciphertext: &[u8], num_candidates: usize) -> Vec<usize> {
    let mut keysize_scores: Vec<(usize, f64)> = (2..=40)
        .filter(|&keysize| ciphertext.len() >= keysize * 4) // Need at least 4 blocks
        .map(|keysize| {
            let avg_distance = calculate_normalized_distance(ciphertext, keysize);
            (keysize, avg_distance)
        })
        .collect();

    keysize_scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    keysize_scores
        .into_iter()
        .take(num_candidates)
        .map(|(keysize, _)| keysize)
        .collect()
}

fn calculate_normalized_distance(ciphertext: &[u8], keysize: usize) -> f64 {
    let blocks: Vec<&[u8]> = ciphertext.chunks(keysize).take(4).collect();
    let mut total_distance = 0f64;
    let mut comparisons = 0;

    // Only compare distinct pairs
    for i in 0..blocks.len() {
        for j in (i + 1)..blocks.len() {
            if blocks[i].len() == blocks[j].len() {
                total_distance += hamming_distance(blocks[i], blocks[j]) as f64;
                comparisons += 1;
            }
        }
    }

    if comparisons == 0 {
        f64::INFINITY
    } else {
        (total_distance / comparisons as f64) / keysize as f64
    }
}

fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    assert_eq!(a.len(), b.len(), "Hamming distance requires equal length inputs");

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        let result = hamming_distance(b"this is a test", b"wokka wokka!!!");
        assert_eq!(result, 37);
    }
}
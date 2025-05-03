use base64::{engine::general_purpose, Engine as _};
fn main() {
    let contents = std::fs::read_to_string("file.txt")
    .unwrap();

    let mut best_candidate: Vec<u8> = Vec::new();
    let mut best_key: u8 = 0;
    let mut best_score: f32 = -9999.0;

    for line in contents.lines() {
        let hex_bytes = hex_to_bytes(line.into());
        for i in 0..=255 {
            let candidate = fixed_xor(&hex_bytes, i);
            let curr_score = score_english_likeness(&candidate);

            if curr_score > best_score {
                best_score = curr_score;
                best_candidate = candidate;
                best_key = i;
            }
        }
    }

    println!("Answer: {} | Score: {} | Key: {}", pretty_print_byte_vec(&best_candidate), best_score, best_key);
}

fn pretty_print_byte_vec(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).unwrap()
}

fn hex_to_bytes(hex_str: String) -> Vec<u8> {
    hex::decode(hex_str).unwrap()
}

fn bytes_to_b64(bytes: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(bytes)
}

fn fixed_xor(bytes: &[u8], key: u8) -> Vec<u8> {
    bytes.iter().map(|byte| {
        byte ^ key
    }).collect()
}

fn score_english_likeness(input: &[u8]) -> f32 {
    let mut score: f32 = 0.0;
    for c in input {
        let char: char = c.clone().into();
        let char_ascii = char.to_ascii_lowercase();
        if char_ascii >= 'a' && char_ascii <= 'z' {
            score += 1.0;
        } else if char_ascii == ' ' || char_ascii == '\n' {
            score += 0.5;
        } else if *c >= 32 && *c <= 126 {
            score += 0.1;
        } else {
            score -= 1.0;
        }
    }

    return score;
}

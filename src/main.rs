// use base64::{engine::general_purpose, Engine as _};
fn main() {
    let mut key_size = 2;
}

fn calc_hamming_distance(str_1: &str, str_2: &str) -> i32 {
    if str_1.len() != str_2.len() {
        panic!("Strings aren't the same length, hamming distance failed");
    }

    let str_1_bits: String = str_1
        .as_bytes()
        .iter()
        .map(|&c| format!("{c:08b}"))
        .collect();
    let str_2_bits: String = str_2
        .as_bytes()
        .iter()
        .map(|&c| format!("{c:08b}"))
        .collect();

    let str_1_bits_bytes: Vec<u8> = str_1_bits.as_bytes().into();
    let str_2_bits_bytes: Vec<u8> = str_2_bits.as_bytes().into();
    let mut hamming_distance = 0;

    for i in 0..str_1_bits_bytes.len() {
        let comp_1 = str_1_bits_bytes[i];
        let comp_2 = str_2_bits_bytes[i];

        if comp_1 != comp_2 {
            hamming_distance += 1;
        }
    }

    hamming_distance
}

// fn pretty_print_byte_vec(bytes: &[u8]) -> &str {
//     std::str::from_utf8(bytes).unwrap()
// }
//

fn repeating_xor(bytes: &[u8], repeating_key: &[u8]) -> Vec<u8> {
    let mut return_vec: Vec<u8> = Vec::new();
    for (i, byte) in bytes.iter().enumerate() {
        let curr_key: u8 = repeating_key[i % repeating_key.len()];
        return_vec.push(byte ^ curr_key);
    }

    return_vec
}

fn hex_to_bytes(hex_str: String) -> Vec<u8> {
    hex::decode(hex_str).unwrap()
}

fn string_to_hex(input: &str) -> String {
    hex::encode(input)
}

// fn bytes_to_b64(bytes: Vec<u8>) -> String {
//     general_purpose::STANDARD.encode(bytes)
// }

// fn fixed_xor(bytes: &[u8], key: u8) -> Vec<u8> {
//     bytes.iter().map(|byte| {
//         byte ^ key
//     }).collect()
// }

// fn score_english_likeness(input: &[u8]) -> f32 {
//     let mut score: f32 = 0.0;
//     for c in input {
//         let char: char = c.clone().into();
//         let char_ascii = char.to_ascii_lowercase();
//         if char_ascii >= 'a' && char_ascii <= 'z' {
//             score += 1.0;
//         } else if char_ascii == ' ' || char_ascii == '\n' {
//             score += 0.5;
//         } else if *c >= 32 && *c <= 126 {
//             score += 0.1;
//         } else {
//             score -= 1.0;
//         }
//     }

//     return score;
// }

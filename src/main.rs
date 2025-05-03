// use base64::{engine::general_purpose, Engine as _};
fn main() {
    let input_string: String = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".into();
    let input_key: String = "ICE".into();

    let hex_str = string_to_hex(&input_string);
    let hex_key = string_to_hex(&input_key);

    let hex_bytes = hex_to_bytes(hex_str);
    let key_bytes = hex_to_bytes(hex_key);

    let answer = repeating_xor(&hex_bytes, &key_bytes);
    println!("Answer: {}", hex::encode(answer));

}

// fn pretty_print_byte_vec(bytes: &[u8]) -> &str {
//     std::str::from_utf8(bytes).unwrap()
// }

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

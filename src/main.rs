use base64::{engine::general_purpose, Engine as _};
fn main() {
    let hex_str: String = "1c0111001f010100061a024b53535009181c".into();
    let key_str: String = "686974207468652062756c6c277320657965".into();

    let hex_bytes = hex_to_bytes(hex_str);
    let key_hex_bytes = hex_to_bytes(key_str);

    let answer = fixed_xor(hex_bytes, key_hex_bytes);

    println!("Answer: {}", hex::encode(answer));
}

fn hex_to_bytes(hex_str: String) -> Vec<u8> {
    hex::decode(hex_str).unwrap()
}

fn bytes_to_b64(bytes: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(bytes)
}

fn fixed_xor(bytes: Vec<u8>, keys: Vec<u8>) -> Vec<u8> {
    // bytes.iter().map(|byte| {
    //     byte ^ key
    // }).collect()

    let mut return_vec: Vec<u8> = Vec::new();
    for (byte, key_byte) in bytes.iter().zip(keys.iter()){
        return_vec.push(byte ^ key_byte);
    };

    return_vec
}

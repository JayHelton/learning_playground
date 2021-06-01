use rand;
use std::cmp::min;
use std::str;
// test
fn generate_key_stream(n: usize) -> Vec<u8> {
    // Not very good random, but good enough
    (0..n).map(|_| rand::random::<u8>()).collect()
}

fn xor_bytes(key_stream: Vec<u8>, message: Vec<u8>) -> Vec<u8> {
    let length = min(key_stream.len(), message.len());
    let mut cipher = Vec::new();
    for i in 0..length {
        cipher.push(message[i] ^ key_stream[i]);
    }
    cipher
}

pub fn run_test() {
    let message = "You Are Awesome".to_string();
    let key_stream = generate_key_stream(message.len());
    let cipher = xor_bytes(key_stream.clone(), message.clone().as_bytes().to_vec());
    let decrypted = xor_bytes(key_stream, cipher.clone());
    assert_eq!(message, str::from_utf8(&decrypted).unwrap().to_string());
    println!(
        "{:?} -> {:?} -> {:?}",
        message,
        cipher,
        str::from_utf8(&decrypted).unwrap()
    );
}

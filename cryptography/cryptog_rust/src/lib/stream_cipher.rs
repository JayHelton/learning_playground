use rand;
use std::cmp::min;
use std::str;

struct KeyStream {
    next: usize,
}

impl KeyStream {
    fn new(seed: usize) -> KeyStream {
        KeyStream { next: seed }
    }

    fn rand<'a>(&'a mut self) -> usize {
        let number: usize = 2;
        self.next = (1103515245 * self.next + 12345) % number.pow(31);
        self.next
    }

    fn get_key_byte<'a>(&'a mut self) -> u8 {
        (self.rand() % 256) as u8
    }
}

fn encrypt<'a>(key_stream: &'a mut KeyStream, message: Vec<u8>) -> Vec<u8> {
    let mut cipher = Vec::new();
    for i in 0..message.len() {
        cipher.push(message[i] ^ key_stream.get_key_byte());
    }
    cipher
}

pub fn run_test() {
    let message = "You Are Awesome".to_string();
    let mut key_stream = KeyStream::new(10);
    let cipher = encrypt(&mut key_stream, message.clone().as_bytes().to_vec());
    let mut key_stream = KeyStream::new(10);
    let decrypted = encrypt(&mut key_stream, cipher.clone());
    assert_eq!(message, str::from_utf8(&decrypted).unwrap().to_string());
    println!(
        "{:?} -> {:?} -> {:?}",
        message,
        cipher,
        str::from_utf8(&decrypted).unwrap()
    );
}

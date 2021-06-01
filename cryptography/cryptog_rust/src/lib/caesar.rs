use std::collections::HashMap;

fn generate_key(n: usize) -> HashMap<char, char> {
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut key = HashMap::new();
    let mut count = 0;
    for c in letters.chars() {
        let i = (count + n) % letters.len();
        key.insert(c, letters.chars().nth(i).unwrap());
        count = count + 1;
    }
    key
}

fn encrypt(key: HashMap<char, char>, message: String) -> String {
    let mut cipher = "".to_string();
    for c in message.chars() {
        if key.contains_key(&c) {
            cipher.push_str(key.get(&c).unwrap().to_string().as_str());
        } else {
            cipher.push_str(&c.to_string().as_str());
        }
    }
    cipher
}

fn get_decrypt_key(key: HashMap<char, char>) -> HashMap<char, char> {
    let mut dkey = HashMap::new();
    for (key, value) in key.into_iter() {
        dkey.insert(value, key);
    }
    dkey
}

pub fn run_test() {
    let key = generate_key(3);
    let message = "YOU ARE AWESOME".to_string();
    let cipher = encrypt(key.clone(), message.clone());
    let dkey = get_decrypt_key(key.clone());
    let decrypted = encrypt(dkey, cipher.clone());
    assert_eq!(message, decrypted);
    println!("{:?} -> {:?} -> {:?}", message, cipher, decrypted);
}

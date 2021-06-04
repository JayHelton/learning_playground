pub fn run_tests() {
    println!("Running Array Functions");
    println!("Reverse String");
    reverse_string("Im a string".to_string());
}

// The point is to reverse without any built ins
fn reverse_string(s: String) {
    let length = s.len() - 1;
    let characters = s.chars();
    let mut result = String::new();
    for i in 0..length + 1 {
        result.push(characters.clone().nth(length - i).unwrap());
    }
    println!("Start: {:?}, Result {:?}", s, result);
}

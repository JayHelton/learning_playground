pub fn run_tests() {
    println!("Running Array Functions");
    println!("Reverse String");
    reverse_string("Im a string".to_string());
    println!("Equal Halves");
    equal_halves(vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9, 1,
    ])
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

fn equal_halves(list: Vec<usize>) {
    for i in list.clone() {
        let first_half = list[0..i].to_vec();
        let last_half = list[i + 1..list.len()].to_vec();
        let added_first_half = first_half.into_iter().reduce(|a, b| a + b);
        let added_last_half = last_half.into_iter().reduce(|a, b| a + b);
        if added_last_half.unwrap() == added_first_half.unwrap() {
            println!("This is the partition we want: {:?}", i);
        }
    }
}

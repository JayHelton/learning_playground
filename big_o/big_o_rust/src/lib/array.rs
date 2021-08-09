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
    let total_len = s.len() - 1;
    let characters = s.chars();
    let mut result = String::new();
    let mut i = total_len;
    while 0 <= i {
        result.push(characters.clone().nth(i).unwrap());
        if i == 0 {
            break;
        }
        i = i - 1;
    }
    println!("Start: {:?}, Result {:?}", s, result);
}

fn equal_halves(list: Vec<usize>) {
    let mut accu = 0;
    for i in list.clone() {
        accu = accu + i;
        let last_half = list[i + 1..list.len()].to_vec();
        let added_last_half = last_half.into_iter().reduce(|a, b| a + b);
        if added_last_half.unwrap() == accu {
            println!("This is the partition we want: {:?}", i);
        }
    }
}

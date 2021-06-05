use std::collections::HashMap;

pub fn run_challenges() {
    let challenges: Vec<fn()> = vec![two_sum];
    for challenge in challenges.clone() {
        challenge();
    }
}

fn two_sum() {
    let nums: Vec<i32> = vec![3, 2, 4];
    let target: i32 = 6;
    println!(
        "Running two_sum with nums: {:?} - target: {:?}",
        nums, target
    );
    let mut dict: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len() {
        let case = target - nums[i];
        if dict.contains_key(&case) {
            println!("Found at {:?} and {:?}", dict.get(&case), i);
        }
        dict.insert(nums[i], i as i32);
    }
}

pub mod lib;
use crate::lib::{caesar, one_time_pad, stream_cipher};

fn main() {
    println!("Caeser Cipher");
    caesar::run_test();
    println!("One-Time Pad");
    one_time_pad::run_test();
    println!("Stream Cipher");
    stream_cipher::run_test();
}

// #[derive(Debug)]
use std::io;

fn main() {
    let mut string = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut string)
        .ok()
        .expect("Couldn't read line"); 
    // string = "hello world";
    let char_vec: Vec<char> = string.trim().chars().collect();
    let char_vec_reverse: Vec<char> = string.trim().chars().rev().collect();
    let char_vec_length = char_vec.len();
    // println!("{:?}",char_vec);
    // println!("{:?}",char_vec_reverse);
    if char_vec == char_vec_reverse{
        println!("This is a palindrome!");
    }
    else {
        println!("This is not a palindrome!");
    }
}
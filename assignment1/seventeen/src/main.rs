// #[derive(Debug)]
use std::io;

fn main() {
    let mut string = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut string)
        .ok()
        .expect("Couldn't read line"); 
    // string = "hello world";
    let length = string.len();
    // println!("Length: {}",length);
    let char_vec: Vec<char> = string.clone().chars().collect();
    let vowels = vec!['a','e','i','o','u','A','E','I','O','U'];
    let mut count:usize = 0;
    for c in char_vec {
        for v in vowels.iter().cloned() {
            if c==v {
                count = count + 1; 
            }
        }
    }
    let consonants = length - 2 - count;
    println!("Vowels: {}\nConsonants: {}",count,consonants)
    // let reverse = string.chars().rev().collect::<String>();
    
}
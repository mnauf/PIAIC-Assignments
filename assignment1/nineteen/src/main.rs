use std::io;

fn main() {
    println!("Enter a string: ");
    let mut string = String::new();
    io::stdin().read_line(&mut string)
        .ok()
        .expect("Couldn't read line"); 
    // string = "hello world";
    // let length = string.len();
    // println!("Length: {}",length);
    let char_vec: Vec<char> = string.clone().chars().collect();
    // let vowels = vec!['a','e','i','o','u','A','E','I','O','U'];
    // let mut count:usize = 0;
    let mut digits = 0;
    let mut alphabets = 0;
    let mut whitespaces = 0;
    for c in char_vec {
        if c.is_digit(10)==true{
            digits = digits + 1;
        };
        if c.is_alphabetic()==true{
            alphabets = alphabets + 1;
        };
        if c.is_whitespace()==true{
            whitespaces = whitespaces + 1;
        };
        };
    let length = string.chars().count();
    println!("Special Characters: {}", length - digits - alphabets - whitespaces);
    println!("Numbers: {}", digits);
    println!("Alphabets: {}", alphabets);
    println!("Spaces: {}", whitespaces-2);
}
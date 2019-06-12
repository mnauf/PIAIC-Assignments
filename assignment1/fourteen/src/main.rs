// #[derive(Debug)]
use std::io;

fn main() {
    let mut string = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut string)
        .ok()
        .expect("Couldn't read line"); 
    let char_vec: Vec<char> = string.trim().chars().collect();
    let char_vec_length = char_vec.len();
    let mut result = 0;
    for c in 0..char_vec_length {
        result = result + char_vec[c].to_string().parse::<u32>().unwrap();
        print!(" {} ",char_vec[c]);
        
        if c >= char_vec_length -1 {
        break
        }
        print!("+");
    }
    print!(" is {}",result);
}
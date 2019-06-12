use std::io;

fn main() {
    println!("Enter a letter");

    let mut letter = String::new();

    io::stdin()
        .read_line(&mut letter)
        .expect("Failed to read line");

    let letter: String = letter
        .trim()
        .parse()
        .expect("Wanted a number");


    let check_array = ["A","E","I","0","U","a","e","i","o","u"];

    let mut counter = 0;

    for i in check_array.iter(){
        
        if letter.trim() == i.trim() {
            counter = counter+1;
        }        
    }

    if counter>0{
            println!("The letter {} is vowel.",letter);
        }
        else {
            println!("The letter {} is not a vowel.",letter);
        }
}
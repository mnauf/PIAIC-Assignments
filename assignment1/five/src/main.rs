use std::io;
// use str::repeat;

fn main() {
    let mut text = String::new();
    println!("Enter a String: ");
    io::stdin().read_line(&mut text)
        .ok()
        .expect("Couldn't read line");    
    let mut number = String::new();
    println!("How many copies of String do you need: ");
    io::stdin().read_line(&mut number)
        .ok()
        .expect("Couldn't read line");    
    let number: usize = number.trim().parse().expect("invalid input"); //why usize?
    let mut counter = 0;
    print!("{} copies of String {} are: ",number,text);
    let result = loop {
        counter += 1;
        print!("{}",text);

    if counter == number {
        break counter;
    }
    };
}
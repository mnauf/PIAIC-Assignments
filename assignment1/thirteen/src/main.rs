use std::io;

fn main() {
    let mut string = String::new();
    println!("Enter a number: ");
    io::stdin().read_line(&mut string)
        .ok()
        .expect("Couldn't read line"); 
    let mut numbers: u32 = string.trim().parse().expect("invalid input");
    let mut add = 0;
    for number in 1..numbers+1{
        add = add + number
    };
    println!("Sum of n positive integers till 5 is: {}",add);
}
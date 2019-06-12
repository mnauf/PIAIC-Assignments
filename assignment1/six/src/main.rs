use std::io;

fn main() {   
    let mut number = String::new();
    println!("Enter a Number: ");
    io::stdin().read_line(&mut number)
        .ok()
        .expect("Couldn't read line");    
    let number: u32 = number.trim().parse().expect("invalid input");
    if number % 2 == 0 {
        println!("{} is Even",number);
    }
    else {
        println!("{} is odd",number);
    }
}
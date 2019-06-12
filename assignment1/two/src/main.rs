use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter an Integer: ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");    
    let input: i32 = input.trim().parse().expect("invalid input");
    if (input > 0){
        println!("Positive Number Entered");
    }
    else if (input ==0) {
        println!("Zero Entered");
    }
    else {
        println!("Negative Number Entered");
    }
}
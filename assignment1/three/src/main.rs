use std::io;

fn main() {
    let mut numerator = String::new();
    println!("Enter Numerator: ");
    io::stdin().read_line(&mut numerator)
        .ok()
        .expect("Couldn't read line");    
    let numerator: i32 = numerator.trim().parse().expect("invalid input");
    let mut denominator = String::new();
    println!("Enter Denominator: ");
    io::stdin().read_line(&mut denominator)
        .ok()
        .expect("Couldn't read line");    
    let denominator: i32 = denominator.trim().parse().expect("invalid input");
    if numerator % denominator == 0 {
        println!("Number {} is completely divisible by Number {}",numerator,denominator);
    }
    else {
        println!("Number {} is not completely divisible by Number {}",numerator,denominator);
    }
}
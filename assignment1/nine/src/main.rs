use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter principal amount: ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");    
    let mut input: f64 = input.trim().parse().expect("invalid input");
    let mut interest = String::new();
    println!("Please enter rate of interest in %: ");
    io::stdin().read_line(&mut interest)
        .ok()
        .expect("Couldn't read line");    
    let interest: f64 = interest.trim().parse().expect("invalid input");
    let mut years = String::new();
    println!("Enter number of years for investment: ");
    io::stdin().read_line(&mut years)
        .ok()
        .expect("Couldn't read line");    
    let years: i32 = years.trim().parse().expect("invalid input");
    let mut counter = 0;
    let mut calc:f64 = input;
    let answer = loop {
        counter +=1;
        calc = calc + (calc * interest);
        if counter == years{
            break calc;
        } 
    };
    println!("After {} years your principal amount {} over an interest rate of {} will be {}",years,input,interest,answer);
}
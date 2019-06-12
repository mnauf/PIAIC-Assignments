use std::io;

fn main() {
    let mut decimal = String::new();
    println!("Enter a decimal: ");
    io::stdin().read_line(&mut decimal)
        .ok()
        .expect("Couldn't read line");
    let mut decimal: i32 = decimal.trim().parse().expect("invalid input");
    println!("{}",to_binary(decimal));
    
}

fn to_binary(mut decimal: i32) -> i32 {
    if decimal == 0 {
        decimal
        } 
    else {
        let mut bits = String::new();
    while decimal > 0 {
        if decimal % 2 == 0 {
            bits.push_str("0");
        } else {
            bits.push_str("1");
        }
        decimal /= 2;
    }
    match bits.chars().rev().collect::<String>().parse() {
            Ok(num) => num,
            Err(_e) => panic!("Something went wrong"),
    }
    }
}
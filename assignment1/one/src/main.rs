use std::io;

fn main() {
    let mut input = String::new();
    println!("Radius of a circle: ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");    
    let input: f64 = input.trim().parse().expect("invalid input");
    let result = 3.142 * input * input;
    println!("Area of a circle, with radius {} is: {}", input,result);
}
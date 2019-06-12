use std::io;

fn main() {
    let mut height = String::new();
    println!("Enter Height in cm: ");
    io::stdin().read_line(&mut height)
        .ok()
        .expect("Couldn't read line");    
    let mut height: f64 = height.trim().parse().expect("invalid input");
    height = height / 100.0;
    let mut weight = String::new();
    println!("Enter Weight in Kg: ");
    io::stdin().read_line(&mut weight)
        .ok()
        .expect("Couldn't read line");    
    let mut weight: f64 = weight.trim().parse().expect("invalid input");
    let result = weight / (height * height);
    println!("Your BMI is: {:.3}",result);
}
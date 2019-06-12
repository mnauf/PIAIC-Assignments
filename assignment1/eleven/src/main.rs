use std::io;

fn main() {
    let mut feet = String::new();
    println!("Enter Height in Feet: ");
    io::stdin().read_line(&mut feet)
        .ok()
        .expect("Couldn't read line");    
    let mut feet: f64 = feet.trim().parse().expect("invalid input");
    let cm = feet * 30.48;
    println!("There are {} Cm in {} ft",cm,feet);
}
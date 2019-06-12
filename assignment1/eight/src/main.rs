use std::io;

fn main() {   
    let mut base = String::new();
    println!("Enter the magnitude of traingle base: ");
    io::stdin().read_line(&mut base)
        .ok()
        .expect("Couldn't read line");    
    let base: f64 = base.trim().parse().expect("invalid input");
    let mut height = String::new();
    println!("Enter the magnitude of traingle base: ");
    io::stdin().read_line(&mut height)
        .ok()
        .expect("Couldn't read line");    
    let height: f64 = height.trim().parse().expect("invalid input");
    let A = 0.5 * base * height;
    println!("Area of Traingle with Height {} and Base {} is {}",height,base,A);
}
use std::io;

fn main() {
    let mut radius = String::new();
    println!("Radius of a Sphere: ");
    io::stdin().read_line(&mut radius)
        .ok()
        .expect("Couldn't read line");    
    let radius: f64 = radius.trim().parse().expect("invalid input");
    let volume = 1.33333333333 * 3.142 * radius * radius * radius;
    println!("Volume of the sphere with radius {} is {:.2}",radius,volume);
}
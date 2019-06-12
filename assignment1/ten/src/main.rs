use std::io;

fn main() {
    let mut x1 = String::new();
    println!("Enter Co-ordinate for x1: ");
    io::stdin().read_line(&mut x1)
        .ok()
        .expect("Couldn't read line");    
    let mut x1: i32 = x1.trim().parse().expect("invalid input");

    let mut x2 = String::new();
    println!("Enter Co-ordinate for x2: ");
    io::stdin().read_line(&mut x2)
        .ok()
        .expect("Couldn't read line");    
    let x2: i32 = x2.trim().parse().expect("invalid input");
    
    let mut y1 = String::new();
    println!("Enter Co-ordinate for y1: ");
    io::stdin().read_line(&mut y1)
        .ok()
        .expect("Couldn't read line");    
    let y1: i32 = y1.trim().parse().expect("invalid input");
    
    let mut y2 = String::new();
    println!("Enter Co-ordinate for y2: ");
    io::stdin().read_line(&mut y2)
        .ok()
        .expect("Couldn't read line");    
    let y2: i32 = y2.trim().parse().expect("invalid input");
    
    let result = ((x2 - x1)^2 + (y2 - y1)^2)^(1/2);
    println!("Distance between points ({},{}) and ({},{}) is {}",x1,y1,x2,y2,result);
}
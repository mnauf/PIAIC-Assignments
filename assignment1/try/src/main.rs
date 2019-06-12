use std::io;
use std::convert::TryFrom;

fn main() {
    let mut binary = String::new();
    // println!("Enter a decimal: ");
    io::stdin().read_line(&mut binary)
        .ok()
        .expect("Couldn't read line");
    let mut binary =binary.to_string();
    println!("{}",binary.len());
    for i in 0..binary.len(){
        println!("{}",2*i);
    }
}
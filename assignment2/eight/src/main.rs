extern crate rand;
use std::io;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    loop{
        let mut x = rng.gen_range(0, 10);
        let mut input = String::new();
        println!("Guess the number: ");
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Couldn't read line");    
        let input: u32 = input.trim().parse().expect("invalid input");
        if input == x {
            println!("That's right! My secret number was {}",x);
}
        else {
            println!("Sorry but I was really thinking of {}",x);
        }
    }
}
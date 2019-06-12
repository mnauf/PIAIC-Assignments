extern crate rand;
use std::io;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut roll_1 = rng.gen_range(0, 10);
    println!("Roll #1: {}",roll_1);
    let mut roll_2 = rng.gen_range(0, 10);
    println!("Roll #2: {}",roll_2);
    let mut add = roll_1 + roll_2;
    println!("The total is {}!",add);
}
extern crate rand;
use std::io;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut guess = rng.gen_range(0, 100);
    println!("I am thinking of a number between 1-100. You have 7 guesses.");
    let mut counter = 1;
    while counter <=7 {
        println!("Guess #{}: ",counter);
        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");    
        let input: u32 = input.trim().parse().expect("invalid input");
        if input == guess {
            println!("You guessed it! What are the odds?!?");
            break;
        }
        else if input > guess {
            println!("You are two high");
        }
        else if input < guess {
            println!("You are two low");
        }
    counter = counter + 1;
    }
    if counter > 7 {
    println!("Sorry you didn't guess it in 7 tries. You lose. The guess was {}",guess);
    }
}
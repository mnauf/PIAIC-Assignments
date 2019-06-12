extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    loop{
        let mut x = rng.gen_range(0, 100);
        if x<=90{
            vec.push(x);
        }
        else {
            break
        }
    };
    vec.sort();
    println!("Sorted vector: {:?}",vec);
    }
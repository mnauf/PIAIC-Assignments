extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for i in 0..100{
    vec.push(rng.gen_range(0.0, 900.0));
    }
    println!("{:?}",vec);
    let mut below_three_hundred = 0;
    let mut below_six_hundred = 0;
    let mut above_six_hundred = 0;
    for i in vec {
        if i < 300.0 {
            below_three_hundred = below_three_hundred + 1;
        }
        else if i >=300.0 && i < 600.0 {
            below_six_hundred = below_six_hundred + 1;
        }
        else {
            above_six_hundred = above_six_hundred + 1;
        }
    }
    println!("Below 300.0: {}",below_three_hundred);
    println!("Below 600.0: {}",below_six_hundred);
    println!("Above 600.0: {}",above_six_hundred);
}

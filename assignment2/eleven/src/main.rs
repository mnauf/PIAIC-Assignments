use std::io;

fn main() {
    println!("I will add up the numbers you give me.");
    let mut calc = 0;
    let mut vec = Vec::new();
    loop {
        println!("Number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");    
        let input: u32 = input.trim().parse().expect("invalid input");
        vec.push(input);
        if input == 0 {
            for i in 0..vec.len(){
                calc = calc + vec[i];
            }
        break;
        }
    };
    println!("The vector is {:?}",vec);
    println!("The total is: {}",calc);
}
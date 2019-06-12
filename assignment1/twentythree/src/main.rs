use std::io;

fn main() {
    let mut date_1 = String::new();
    println!("Enter a date in (dd/mm/yy) format: ");
    io::stdin().read_line(&mut date_1)
        .ok()
        .expect("Couldn't read line");

    let mut date_2 = String::new();
    println!("Enter a date in (dd/mm/yy) format: ");
    io::stdin().read_line(&mut date_2)
        .ok()
        .expect("Couldn't read line");
    let mut date_1 = date_1.split("/");
    let mut date_2 = date_2.split("/"); 
    // let mut date_1 = "22/8/2019".split("/");
    // let mut date_2 = "30/8/2019".split("/");
    let vec_1: Vec<&str> = date_1.collect();
    let vec_2: Vec<&str> = date_2.collect();
    println!("{:#?}",vec_1);
    println!("{:#?}",vec_2);
    let my_int_2 = vec_2[0].parse::<i32>().unwrap();
    let my_int_1 = vec_1[0].parse::<i32>().unwrap();
    let result = my_int_2 - my_int_1;
    println!("The difference between two dates is: {}",result);
    }
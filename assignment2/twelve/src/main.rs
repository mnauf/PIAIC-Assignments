use std::io;

fn main() {
    println!("Give me a mathematical expression: ");
    while true {
        println!("> ");
        let mut input = String::new();
        let mut result = String::from("");
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("Couldn't read line");

        for c in input.chars() {
            match c.to_string().as_ref() {
                "*" => result = multiply(&input),
                "+" => result = add(&input),
                "-" => result = subtract(&input),
                "/" => result = divide(&input),
                "^" => result = exponent(&input),
                _ => print!(""),
            }
        }
        let mut result_1 = String::from("");
        let vec_1: Vec<char> = input.trim().chars().collect();
        if vec_1[0].to_string().parse::<f64>().unwrap() == 0.0 {
            println!("Exiting...");
            break;
        }
        println!("{}", result);
    }

}
fn multiply(input: &String) -> String {
    let mut result_1 = String::from("");
    let vec_1: Vec<char> = input.trim().chars().collect();
    if vec_1.len() == 3 {
        format!(
            "{:.2}",
            vec_1[0].to_string().parse::<f64>().unwrap()
                * vec_1[2].to_string().parse::<f64>().unwrap()
        )
    } else if vec_1.len() == 4 {
        result_1.push_str(vec_1[0].to_string().as_ref());
        result_1.push_str(vec_1[1].to_string().as_ref());
        format!(
            "{:.2}",
            result_1.parse::<f64>().unwrap() * vec_1[3].to_string().parse::<f64>().unwrap()
        )
    } else {
        format!("Couldn't evaluate. Try again!")
    }
}
fn add(input: &String) -> String {
    let mut result_1 = String::from("");
    let vec_1: Vec<char> = input.trim().chars().collect();
    if vec_1.len() == 3 {
        format!(
            "{:.2}",
            vec_1[0].to_string().parse::<f64>().unwrap()
                + vec_1[2].to_string().parse::<f64>().unwrap()
        )
    } else if vec_1.len() == 4 {
        result_1.push_str(vec_1[0].to_string().as_ref());
        result_1.push_str(vec_1[1].to_string().as_ref());
        format!(
            "{:.2}",
            result_1.parse::<f64>().unwrap() + vec_1[3].to_string().parse::<f64>().unwrap()
        )
    } else {
        format!("Couldn't evaluate. Try again!")
    }
}
fn subtract(input: &String) -> String {
    let mut result_1 = String::from("");
    let vec_1: Vec<char> = input.trim().chars().collect();
    if vec_1.len() == 3 {
        format!(
            "{:.2}",
            vec_1[0].to_string().parse::<f64>().unwrap()
                - vec_1[2].to_string().parse::<f64>().unwrap()
        )
    } else if vec_1.len() == 4 {
        result_1.push_str(vec_1[0].to_string().as_ref());
        result_1.push_str(vec_1[1].to_string().as_ref());
        format!(
            "{:.2}",
            result_1.parse::<f64>().unwrap() - vec_1[3].to_string().parse::<f64>().unwrap()
        )
    } else {
        format!("Couldn't evaluate. Try again!")
    }
}
fn divide(input: &String) -> String {
    let mut result_1 = String::from("");
    let vec_1: Vec<char> = input.trim().chars().collect();
    if vec_1.len() == 3 {
        format!(
            "{:.2}",
            vec_1[0].to_string().parse::<f64>().unwrap()
                / vec_1[2].to_string().parse::<f64>().unwrap()
        )
    } else if vec_1.len() == 4 {
        result_1.push_str(vec_1[0].to_string().as_ref());
        result_1.push_str(vec_1[1].to_string().as_ref());
        format!(
            "{:.2}",
            result_1.parse::<f64>().unwrap() / vec_1[3].to_string().parse::<f64>().unwrap()
        )
    } else {
        format!("Couldn't evaluate. Try again!")
    }

}

fn exponent(input: &String) -> String {
    let mut result_1 = String::from("");
    let vec_1: Vec<char> = input.trim().chars().collect();
    if vec_1.len() == 3 {
        format!(
            "{:.2}",
            vec_1[0]
                .to_string()
                .parse::<u32>()
                .unwrap()
                .pow(vec_1[2].to_string().parse::<f64>().unwrap() as u32)
        )
    } else {
        format!("Couldn't evaluate. Try again!")
    }
}
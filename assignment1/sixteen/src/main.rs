use std::io;

fn main() {
    let mut binary = String::new();
    println!("Enter a decimal: ");

    io::stdin().read_line(&mut binary)
        .expect("Couldn't read line");

    match to_decimal(binary) {
        Some(num) => println!("{}", num),
        None => println!("Not a binary!")
    }
}

fn to_decimal(binary_str:String) -> Option<i64> {
    let binary_str = binary_str.trim();

    let mut out_num = 0;
    for c in binary_str.chars() {
        match c {
            '0' => out_num = out_num * 2,
            '1' => out_num = out_num * 2 + 1,
            _ => return None
        }
    }

    Some(out_num)
}
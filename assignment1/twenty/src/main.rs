fn main() {
    let mut count = 0;
    loop {
        count = count + 1;
        print!("{}","* ".repeat(count));
        if count == 5{
            break;
        }
        println!("");
    }
    println!("");
    loop {
    count = count - 1;
    print!("{}","* ".repeat(count));
    if count == 0{
        break;
    }
    println!("");
}
}
fn main() {
        let mut count_A = 0;
    
        loop {
        count_A = count_A + 1;
        let mut count_B = 0;
        loop{
            count_B = count_B + 1;
            print!("{}",count_B); //count_A
            print!(" ");
            if count_A == count_B{
                break;
            }
        }
        if count_A == 5{
            break;
        }
        println!("");
    }
    println!("");
        loop {
        count_A = count_A - 1;
        let mut count_B = 0;
        loop{
            count_B = count_B + 1;
            print!("{}",count_B); //count_A
            print!(" ");
            if count_A == count_B{
                break;
            }
        }
        if count_A == 1{
            break;
        }
        println!("");
    }
}
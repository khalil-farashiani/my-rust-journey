use std::io;
use rand::Rng;
use std::cmp::Ordering;

//this is a official example of the book (The Rust Programming Language Book)
fn main() {
    
    let random_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut user_input = String::new();
        io::stdin()
        .read_line(&mut user_input)
        .expect("error in get user input");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_input.cmp(&random_number) {
            Ordering::Equal => {
                println!("NICE!!");
                break;
        }
            Ordering::Greater => println!("Too Big!!"),
            Ordering::Less => println!("Too Small!")
            
        }
    }
}

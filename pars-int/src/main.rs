use std::io;

fn main() {
    let mut a_number = String::new();
    io::stdin()
    .read_line(&mut a_number)
    .expect("invalid input");

    // Rust allows us to shadow the previous value of a let
    let a_number: u32 = a_number.trim().parse().expect("input is not a valid number");

    println!("{a_number}")
}
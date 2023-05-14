use std::io;

// get the string data and print in std output using io package
fn main() {
    let mut entered = String::new();

    io::stdin()
        .read_line(&mut entered)
        .expect("Failed to read line");

    println!("You entered: {entered}");
}
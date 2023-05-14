use std::cmp::Ordering;

fn main() {
    let number_1: i32 = 85;
    let number_2: i32 = 9;
    match number_1.cmp(&number_2) {
        Ordering::Less => println!("state one"),
        Ordering::Greater => println!("state two"),
        Ordering::Equal => println!("state three")
    }
}
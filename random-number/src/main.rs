use rand::Rng;

// generate a random number with Rng
fn main(){
    println!("generate a random number in specefic range");
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("{random_number}");
}
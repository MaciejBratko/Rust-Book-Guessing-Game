use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut rng = rand::rng(); // Create a random number generator
    let random_number = rng.random_range(1..=100); // Generate a random number between 1 and 100
    println!("Random number: {}", random_number);
}

use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn get_user_guess() -> i32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to fetch the line.");
    guess.trim().parse().expect("Please enter a valid number!")
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries = 0;
    println!("Guess the number!");

    loop {
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        tries += 1;
        let guess = get_user_guess();

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low! Try again!\n"),
            Ordering::Greater => println!("Too high! Try again!\n"),
            Ordering::Equal => {
                println!("\nYou win! It took you {tries} tries to guess the number!");
                break;
            }
        }
    }

    println!("The secret number is: {}", secret_number);
}
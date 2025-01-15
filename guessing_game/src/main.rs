use rand::Rng;
use std::cmp::Ordering;
use std::io::{self};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut tries = 0;
    println!("Guess the number!");

    let mut win = false;

    while !win {
        println!("Your guess: ");

        let mut guess = String::new();

        tries += 1;

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to fetch the line.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low! Try again!"),
            Ordering::Greater => println!("Too high! Try again!"),
            Ordering::Equal => {
                println!("You win! It took you {} tries to guess the number!", tries);
                win = true;
            }
        }
    }

    println!("The secret number is: {}", secret_number);
}

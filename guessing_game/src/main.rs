use std::cmp::Ordering;
use std::io;
mod time_rand;
use time_rand::generate_random_number;

fn main() {
    let secret_number = (generate_random_number(100) + 1) as i32;
    println!("The secret number is: {}", secret_number);
    println!("Guess the number!");

    let mut win = false;

    while !win {
        println!("You guess: ");

        let mut guess = String::new();

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

        println!("Try again!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                win = true;
            }
        }
    }

    println!("The secret number is: {}", secret_number);
}

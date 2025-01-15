use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use rand::Rng;
use std::fs::{read_to_string, OpenOptions};
use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;

fn get_or_set_range(set_new: bool) -> Vec<i32> {
    let file_path = "range.txt";

    // Attempt to read numbers from the file if not setting a new range
    if !set_new {
        return match read_to_string(file_path) {
            Ok(content) => {
                if content.trim().is_empty() {
                    println!("The range file is empty. Using default range.");
                    vec![10, -20, 30, -40, 50]
                } else {
                    // Parse the content as a list of numbers
                    let numbers: Result<Vec<i32>, _> = content
                        .trim()
                        .split(',')
                        .map(|num| num.parse::<i32>())
                        .collect();
                    numbers.unwrap_or_else(|_| {
                        println!(
                            "Failed to parse numbers from the range file. Using default range."
                        );
                        vec![10, -20, 30, -40, 50]
                    })
                }
            }
            Err(_) => {
                println!("Failed to read the range file. Using default range.");
                vec![10, -20, 30, -40, 50]
            }
        };
    }

    // If setting a new range or file reading/parsing failed, prompt the user
    if set_new {
        println!(
            "Enter the numbers to randomize from, separated by commas (e.g., '-10,20,30,-5'): "
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let numbers: Result<Vec<i32>, _> = input
            .trim()
            .split(',')
            .map(|num| num.parse::<i32>())
            .collect();

        return match numbers {
            Ok(nums) => {
                // Save the array to the file for future use
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(file_path)
                    .expect("Failed to open file");

                writeln!(
                    file,
                    "{}",
                    nums.iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
                .expect("Failed to write to file");

                nums
            }
            Err(_) => {
                println!("Invalid input. Using default range.");
                vec![10, -20, 30, -40, 50]
            }
        };
    }

    // Default fallback array
    vec![10, -20, 30, -40, 50]
}

fn main() {
    let file_path = "range.txt";

    // Read the current range from range.txt and display it
    match read_to_string(file_path) {
        Ok(content) => {
            if !content.trim().is_empty() {
                println!("Current range: {}", content);
            } else {
                println!("The range file is empty.");
            }
        }
        Err(_) => println!("Failed to read the range file."),
    }

    println!("Select an option:");
    println!("1. Run with saved range (default if not found)");
    println!("2. Set range and run");
    print!("Enter your choice: ");
    stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let set_new = choice.trim() == "2";

    let numbers = get_or_set_range(set_new);
    let mut rng = rand::thread_rng();
    let mut stdout = stdout();

    for i in 0..49 {
        let value = numbers[rng.gen_range(0..numbers.len())];
        execute!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print(format!("Value: {}\n", value)),
            ResetColor
        )
        .expect("Failed to execute command");
        let delay = (50.0 + (450.0 * (i as f64 / 48.0).powf(2.0))) as u64; // Quadratic increase from 50ms to 500ms
        thread::sleep(Duration::from_millis(delay));
    }
    let final_value = numbers[rng.gen_range(0..numbers.len())];
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(format!("Final value: {}\n", final_value)),
        ResetColor
    )
    .expect("Failed to execute command");

    println!("Press Enter to close...");
    io::stdin().read_line(&mut String::new()).unwrap(); // Wait for Enter key press
}

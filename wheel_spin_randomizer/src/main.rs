use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use rand::Rng;
use std::error::Error;
use std::fs::{read_to_string, OpenOptions};
use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;

const DEFAULT_NUMBERS: &[&str] = &["10", "-20", "30", "-40", "50"];

fn get_or_set_range(set_new: bool) -> Result<Vec<String>, Box<dyn Error>> {
    let file_path = "range.txt";

    if !set_new {
        // Attempt to read numbers from the file
        let content = match read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                println!("Failed to read the range file: {}. Using default range.", e);
                return Ok(DEFAULT_NUMBERS.iter().map(|s| s.to_string()).collect());
            }
        };
        return if content.trim().is_empty() {
            println!("The range file is empty. Using default range.");
            Ok(DEFAULT_NUMBERS.iter().map(|s| s.to_string()).collect())
        } else {
            let numbers: Vec<String> = content
                .trim()
                .split(',')
                .map(|num| num.trim().to_string())
                .filter(|num| !num.is_empty())
                .collect();

            if numbers.is_empty() {
                println!("No valid entries found in the range file. Using default range.");
                return Ok(DEFAULT_NUMBERS.iter().map(|s| s.to_string()).collect());
            }

            Ok(numbers)
        };
    }

    // Prompt the user to set a new range
    println!("Enter the strings to randomize from, separated by commas (e.g., 'a,b,c,d'): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let numbers: Vec<String> = input
        .trim()
        .split(',')
        .map(|num| num.trim().to_string())
        .filter(|num| !num.is_empty())
        .collect();

    if numbers.is_empty() {
        println!("Invalid input. Using default range.");
        Ok(DEFAULT_NUMBERS.iter().map(|s| s.to_string()).collect())
    } else {
        // Save the array to the file for future use
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)?;

        writeln!(file, "{}", numbers.join(","))?;

        Ok(numbers)
    }
}

fn main() {
    // Initialize logging
    env_logger::init();

    let file_path = "range.txt";

    // Read the current range from range.txt and display it
    match read_to_string(file_path) {
        Ok(content) => {
            if !content.trim().is_empty() {
                println!("Current range: {}", content);
            } else {
                log::info!("The range file is empty.");
            }
        }
        Err(e) => {
            log::error!("Failed to read the range file: {}", e);
            println!("Failed to read the range file: {}.", e);
        }
    }

    println!("Select an option:");
    println!("1. Run with saved range (default if not found)");
    println!("2. Set range and run");
    print!("Enter your choice: ");
    stdout().flush().unwrap();

    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_err() {
        log::error!("Failed to read line from input.");
        std::process::exit(1);
    }
    let set_new = choice.trim() == "2";

    let numbers = get_or_set_range(set_new).unwrap_or_else(|e| {
        println!("An error occurred while getting the range: {}", e);
        DEFAULT_NUMBERS.iter().map(|s| s.to_string()).collect()
    });

    let mut rng = rand::thread_rng();
    let mut stdout = stdout();

    for i in 0..49 {
        let value = &numbers[rng.gen_range(0..numbers.len())];
        if execute!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print(format!("Value: {}\n", value)),
            ResetColor
        )
        .is_err()
        {
            log::error!("Failed to execute terminal command.");
            continue; // Decide whether to continue with the next iteration
        }
        let delay = (50.0 + (450.0 * (i as f64 / 48.0).powf(2.0))) as u64; // Quadratic increase from 50ms to 500ms
        thread::sleep(Duration::from_millis(delay));
    }
    let final_value = &numbers[rng.gen_range(0..numbers.len())];
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(format!("Final value: {}\n", final_value)),
        ResetColor
    )
    .expect("Failed to execute command");

    println!("Press Enter to close...");
    let _ = io::stdin().read_line(&mut String::new()); // Wait for Enter key press
}

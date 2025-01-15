use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use rand::Rng;
use std::fs::{read_to_string, OpenOptions};
use std::io::{self, stdout, Write};
use std::thread;
use std::time::Duration;

fn get_or_set_range(set_new: bool) -> Vec<String> {
    let file_path = "range.txt";

    // Attempt to read numbers from the file if not setting a new range
    if !set_new {
        return match read_to_string(file_path) {
            Ok(content) => {
                if content.trim().is_empty() {
                    println!("The range file is empty. Using default range.");
                    vec![
                        String::from("10"),
                        String::from("-20"),
                        String::from("30"),
                        String::from("-40"),
                        String::from("50"),
                    ]
                } else {
                    // Split the content as a list of strings
                    content
                        .trim()
                        .split(',')
                        .map(|num| num.trim().to_string())
                        .collect()
                }
            }
            Err(_) => {
                println!("Failed to read the range file. Using default range.");
                vec![
                    String::from("10"),
                    String::from("-20"),
                    String::from("30"),
                    String::from("-40"),
                    String::from("50"),
                ]
            }
        };
    }

    // If setting a new range or file reading/parsing failed, prompt the user
    if set_new {
        println!("Enter the strings to randomize from, separated by commas (e.g., 'a,b,c,d'): ");
        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("Error reading line: {:?}", e);
            std::process::exit(1);
        }

        let numbers: Vec<String> = input
            .trim()
            .split(',')
            .map(|num| num.trim().to_string())
            .filter(|num| !num.is_empty())
            .collect();

        if numbers.is_empty() {
            println!("Invalid input. Using default range.");
            return vec![
                String::from("10"),
                String::from("-20"),
                String::from("30"),
                String::from("-40"),
                String::from("50"),
            ];
        }

        // Save the array to the file for future use
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open file: {e}");
                std::process::exit(1);
            }
        };

        match writeln!(file, "{}", numbers.join(",")) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Failed to write to file: {}", e);
                std::process::exit(1);
            }
        }

        return numbers;
    }

    // Default fallback array
    vec![
        String::from("10"),
        String::from("-20"),
        String::from("30"),
        String::from("-40"),
        String::from("50"),
    ]
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
        Err(e) => println!("Failed to read the range file: {}.", e),
    }

    println!("Select an option:");
    println!("1. Run with saved range (default if not found)");
    println!("2. Set range and run");
    print!("Enter your choice: ");
    stdout().flush().unwrap();

    let mut choice = String::new();
    if io::stdin().read_line(&mut choice).is_err() {
        println!("Failed to read line from input.");
        std::process::exit(1);
    }
    let set_new = choice.trim() == "2";

    let numbers = get_or_set_range(set_new);
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
            println!("Failed to execute terminal command.");
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

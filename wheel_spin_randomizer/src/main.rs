use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use rand::Rng;
use std::io::stdout;
use std::thread;
use std::time::Duration;

fn main() {
    let numbers = vec!["1", "-2", "3", "-4", "5"];
    let spin_count = 49;
    let mut rng = rand::thread_rng();
    let mut stdout = stdout();

    for i in 0..spin_count {
        let selected_value = numbers[rng.gen_range(0..numbers.len())];
        
        execute!(
            stdout,
            SetForegroundColor(Color::Blue),
            Print(format!("Value: {}\n", selected_value)),
            ResetColor
        ).unwrap();

        let delay = (50.0 + (450.0 * (i as f64 / (spin_count - 1) as f64).powf(2.0))) as u64;
        thread::sleep(Duration::from_millis(delay));
    }

    let final_value = numbers[rng.gen_range(0..numbers.len())];
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(format!("Final value: {}\n", final_value)),
        ResetColor
    ).unwrap();
}

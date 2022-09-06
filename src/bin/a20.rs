// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

enum Input {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl Input {
    fn get_input_enum(input: &str) -> Option<Self> {
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "off" => Some(Input::Off),
            "sleep" => Some(Input::Sleep),
            "shutdown" => Some(Input::Shutdown),
            "reboot" => Some(Input::Reboot),
            "hibernate" => Some(Input::Hibernate),
            _ => None,
        }
    }
}

fn get_message(input: Input) {
    use Input::*;

    match input {
        Off => println!("switching off"),
        Sleep => println!("sleeping"),
        Reboot => println!("rebooting"),
        Shutdown => println!("shuting down"),
        Hibernate => println!("Hibernating"),
    }
}

use std::io;

fn main() {
    let mut buffer = String::new();

    let user_input_status = io::stdin().read_line(&mut buffer);

    if user_input_status.is_ok() {
        match Input::get_input_enum(&buffer) {
            Some(input) => get_message(input),
            None => println!("invalid input"),
        }
    } else {
        println!("Error reading input");
    }
}

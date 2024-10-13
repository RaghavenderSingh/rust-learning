// Import necessary libraries
use colored::*; // For colored console output
use rand::Rng; // For generating random numbers
use std::cmp::Ordering; // For comparing numbers
use std::io; // For input/output operations

fn main() {
    println!("{}", "Welcome to the Guessing Game!".bright_purple().bold());
    // Generate a random number between 1 and 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Start an infinite loop for the game
    loop {
        println!("{}", "Guess the number".cyan());
        println!("{}", "Please input your guess:".yellow());

        // Create a new empty String to store the user's guess
        let mut guess = String::new();

        // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the string input to a u32 (unsigned 32-bit integer)
        // If parsing fails, continue to the next iteration of the loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please enter a valid number!".red());
                continue;
            }
        };

        println!("Your guess: {}", guess.to_string().bright_blue());

        // Compare the guess with the secret number
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "You win!".green().bold());
                break; // Exit the loop if the guess is correct
            }
            Ordering::Greater => println!("{}", "Too Big!".red()),
            Ordering::Less => println!("{}", "Too Small!".red()),
        }
    }
}

# Rust Guessing Game

A simple and interactive number guessing game implemented in Rust. The game generates a random number between 1 and 100, and the player tries to guess it with helpful feedback after each attempt.

## Features

- Random number generation
- User input handling with validation
- Colored console output for enhanced user experience
- Attempt counter

## Prerequisites

Ensure you have Rust installed on your system. If not, install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation

1. Clone the repository:

   ```
   
   git clone https://github.com/yourusername/rust-learning/gussing_game
   cd rust-guessing-game
   ```

2. Build the project:
   ```
   cargo build
   ```

## Running the Game

To start the game, run:

```
cargo run
```

Follow the on-screen prompts to play.

## How to Play

1. The game will prompt you to guess a number between 1 and 100.
2. Enter your guess and press Enter.
3. You'll receive feedback: "Too small!", "Too big!", or "You win!".
4. Keep guessing until you find the correct number.
5. The game will display the number of attempts you took to win.

## Project Structure

- `src/main.rs`: Contains the main game logic.
- `Cargo.toml`: The manifest file for Rust's package manager, cargo.

## Dependencies

This project uses the following external crates:

- `rand`: For generating random numbers.
- `colored`: For adding color to the console output.

These dependencies are listed in the `Cargo.toml` file and will be automatically downloaded when you build the project.

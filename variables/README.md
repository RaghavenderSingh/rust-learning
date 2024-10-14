# Rust Variables Demo

A simple demonstration of variable concepts in Rust, including immutability, mutability, shadowing, and constants.

## Features

- Immutable variables
- Variable shadowing
- Mutable variables
- Constants
- Print statements to display variable values

## Prerequisites

Ensure you have Rust installed on your system. If not, install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/rust-variables-demo
   cd rust-variables-demo
   ```

2. Build the project:
   ```
   cargo build
   ```

## Running the Demo

To run the demonstration, execute:

```
cargo run
```

The program will display the values of variables at different stages, illustrating Rust's variable concepts.

## Code Explanation

The demo covers the following concepts:

1. Immutable variables:
   ```rust
   let x = 10;
   ```

2. Variable shadowing:
   ```rust
   let x = 10;
   let x = "six";
   ```

3. Mutable variables:
   ```rust
   let mut y = 10;
   y = 30;
   ```

4. Constants:
   ```rust
   const RUST_CONST_VARIABLE: u32 = 30;
   ```

## Project Structure

- `src/main.rs`: Contains the main demonstration code.
- `Cargo.toml`: The manifest file for Rust's package manager, cargo.

## Dependencies

This project doesn't use any external crates. It only uses Rust's standard library.

## Learning Outcomes

After running and studying this demo, you should understand:

- How Rust handles variable immutability by default
- The concept of shadowing and how it differs from mutability
- How to create and use mutable variables
- How to declare and use constants in Rust

This demonstration serves as a foundation for understanding Rust's more advanced concepts like ownership and borrowing.
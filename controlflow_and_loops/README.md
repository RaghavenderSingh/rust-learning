# Rust Control Flow and Loop Demo

This project demonstrates various control flow constructs in Rust, including if-else statements, loops, and iterations.

## Features

- If-else statements
- Using if in a let statement
- Simple loop with a break condition
- While loop
- For loop with array iteration and range

## Prerequisites

Ensure you have Rust installed on your system. If not, install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/rust-learning/controlflow_and_loops
   cd rust-control-flow-demo
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

The program will run through various control flow examples, demonstrating different Rust constructs.

## Code Explanation

The demo covers the following concepts:

1. If-else statements:

   ```rust
   if number == 10 {
       println!("The number is equal to 10");
   } else if number < 10 {
       println!("The number is smaller than 10")
   } else {
       println!("The number is greater than 10")
   }
   ```

2. Using if in a let statement:

   ```rust
   let number = if condition { 5 } else { 6 };
   ```

3. Simple loop with a break condition:

   ```rust
   let simple_loop = loop {
       counter += 1;
       if counter == 10 {
           break counter;
       }
   };
   ```

4. While loop:

   ```rust
   while number != 10 {
       println!("{}!", number);
       number += 1;
   }
   ```

5. For loop with array iteration and range:

   ```rust
   for element in a.iter() {
       println!("the value is: {}", element);
   }

   for range in 1..4 {
       println!("the range is: {}", range);
   }
   ```

## Project Structure

- `src/main.rs`: Contains the main demonstration code with examples of different control flow constructs.
- `Cargo.toml`: The manifest file for Rust's package manager, cargo.

## Dependencies

This project doesn't use any external crates. It only uses Rust's standard library.

## Learning Outcomes

After running and studying this demo, you should understand:

- How to use if-else statements in Rust
- How to use if expressions in let statements
- How to create and break out of loops
- How to use while loops for conditional repetition
- How to use for loops for iterating over collections and ranges

This demonstration serves as a foundation for understanding Rust's control flow constructs, which are crucial for writing efficient and expressive Rust code.

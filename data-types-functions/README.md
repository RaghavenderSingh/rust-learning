# Rust Data Types and Functions Demo

A comprehensive demonstration of Rust's primary data types and basic function concepts, including scalar types, compound types, and different function return methods.

## Features

- Scalar data types (integers, floating-point numbers, booleans, characters)
- Compound data types (tuples, arrays)
- Function definitions and calls
- Function parameters and return values
- Different methods of returning values from functions

## Prerequisites

Ensure you have Rust installed on your system. If not, install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/rust-learning/data-types-functions
   cd data-types-functions
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

The program will demonstrate various data types and function concepts in Rust.

## Code Explanation

The demo covers the following concepts:

1. Scalar Data Types:

   ```rust
   let a = 98_222;  // Integer
   let f = 2.0;     // Floating-point
   let t = true;    // Boolean
   let c = 'z';     // Character
   ```

2. Compound Data Types:

   ```rust
   let tup = ("Love you", 3000);  // Tuple
   let error_code = [200, 404, 500];  // Array
   ```

3. Function Definitions:

   ```rust
   fn my_function() {
       println!("Another function.")
   }
   ```

4. Function Parameters:

   ```rust
   fn function_with_params(x: i32, y: i32) {
       println!("The value of x: {}", x);
       println!("The value of y: {}", y);
   }
   ```

5. Function Return Values:

   ```rust
   fn return_function_type_1(x: i32, y: i32) -> i32 {
       let z = x + y;
       return z;
   }

   fn return_function_type_2(x: i32, y: i32) -> i32 {
       x + y
   }
   ```

## Project Structure

- `src/main.rs`: Contains the main demonstration code.
- `Cargo.toml`: The manifest file for Rust's package manager, cargo.

## Dependencies

This project doesn't use any external crates. It only uses Rust's standard library.

## Learning Outcomes

After running and studying this demo, you should understand:

- Rust's four primary scalar types: integers, floating-point numbers, booleans, and characters
- How to use compound types like tuples and arrays in Rust
- How to define and call functions in Rust
- How to specify function parameters and return types
- Different ways to return values from functions in Rust
- Rust's concept of expressions vs. statements, particularly in function returns

This demonstration serves as a foundation for understanding Rust's type system and function syntax, which are crucial for writing efficient and safe Rust code.

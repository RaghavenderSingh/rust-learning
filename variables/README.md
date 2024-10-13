# Rust Variables Demo

This project demonstrates basic variable concepts in Rust, including immutability, mutability, shadowing, and constants.

## Overview

The main program showcases the following Rust concepts:

1. Immutable variables
2. Variable shadowing
3. Mutable variables
4. Constants

## Code Explanation

### Immutable Variables

```rust
let x = 10;
```

By default, variables in Rust are immutable. Once a value is bound to a name, you can't change that value.

### Shadowing

```rust
let x = 10;
// ...
let x = "six";
```

Rust allows you to declare a new variable with the same name as a previous variable. This is known as shadowing. It's different from marking a variable as mutable.

### Mutable Variables

```rust
let mut y = 10;
y = 30;
```

To make a variable mutable, we use the `mut` keyword. This allows you to change the value bound to the variable.

### Constants

```rust
const RUST_CONST_VARIABLE: u32 = 30;
```

Constants in Rust are always immutable and must be type annotated. By convention, they are named using SCREAMING_SNAKE_CASE.

## Running the Program

To run this program:

1. Ensure you have Rust installed on your system.
2. Save the code in a file with a `.rs` extension (e.g., `main.rs`).
3. Open a terminal and navigate to the directory containing the file.
4. Run the following command:

   ```
   rustc main.rs
   ./main
   ```

This will compile and run the program, displaying the output in the terminal.

## Learning Outcomes

After studying this code, you should understand:

- The default immutability of variables in Rust
- How to use shadowing to reuse variable names
- How to create mutable variables
- How to declare and use constants

This demo serves as a basic introduction to variable handling in Rust, which is fundamental to understanding Rust's ownership and borrowing concepts.

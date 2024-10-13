fn main() {
    let x = 10;
    // Variables in Rust are immutable by default
    // x = 20; // This would cause a compile-time error
    println!("The initial value of x is: {}", x);

    // This is called shadowing: we can redeclare 'x' with a new type and value
    // Shadowing preserves immutability while allowing us to reuse the variable name
    let x = "six";
    println!("The value of x after shadowing is: {}", x);

    let mut y = 10;
    println!("The initial value of y is: {}", y);

    // We use the "mut" keyword to make variables mutable
    y = 30;
    println!("The value of y after change is: {}", y);

    // Constants must be type-annotated and are always immutable
    // By convention, constant names are in SCREAMING_SNAKE_CASE
    const RUST_CONST_VARIABLE: u32 = 30;
    println!(
        "The value of RUST_CONST_VARIABLE is: {}",
        RUST_CONST_VARIABLE
    );

    // Constants cannot be mutable
    // const mut RUST_CONST_VARIABLE2: u32 = 30; // This would cause a compile-time error
}

fn main() {
    // Rust has four primary scalar data types:

    // 1. Integer
    // Integers are whole numbers without a fractional component.
    // Rust provides signed (i8, i16, i32, i64, i128, isize) and unsigned (u8, u16, u32, u64, u128, usize) integer types.
    // The number in the type name indicates the number of bits that type uses.
    let a = 98_222; // Decimal (underscores can be used as visual separators for readability)
    let b = 0xff; // Hexadecimal (base 16, prefixed with 0x)
    let c = 0o77; // Octal (base 8, prefixed with 0o)
    let d = 0b1111_0000; // Binary (base 2, prefixed with 0b)
    let e = b'A'; // Byte (u8 only, represents ASCII character 'A', which is 65 in decimal)

    // 2. Floating-point numbers
    // Rust has two types of floating-point numbers: f32 (32 bits) and f64 (64 bits).
    // These follow the IEEE-754 standard for floating-point computation.
    let f = 2.0; // By default, floating-point literals are f64
    let g: f32 = 3.0; // Explicitly specified as f32. Note the type annotation `: f32`

    // Arithmetic operations can be performed on numeric types
    // The type of `sum` will be inferred based on the types of `a` and `b`
    let sum = a + b; // Addition. Other operations: - (subtraction), * (multiplication), / (division), % (modulus)

    // 3. Boolean
    // The boolean type in Rust has two possible values: true and false.
    // Booleans are often used in conditional statements and are one byte in size.
    let t = true;
    let f = false;

    // 4. Character
    // The char type represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
    // A char is always 4 bytes in size and uses single quotes.
    let c = 'z'; // This could be any Unicode character, not just ASCII

    // Note: Variables are immutable by default in Rust.
    // This is part of Rust's focus on safety and concurrency.
    // To make a variable mutable, you must use the 'mut' keyword, like this:
    // let mut mutable_variable = 5;

    // Type inference: Rust can often infer the type of a variable based on how it's used,
    // but explicit type annotations (like with `g` above) can be used for clarity or when necessary.

    // Integer overflow: In debug builds, Rust includes checks for integer overflow that cause the program to panic at runtime.
    // In release builds with the --release flag, Rust performs two's complement wrapping.

    // Floating-point precision: Be aware that floating-point numbers can have precision issues,
    // especially when comparing them for equality.

    //---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

    // Compound Types
    // Rust has two primitive compound types: tuples and arrays

    // Tuples
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup = ("Love you", 3000);
    // We can use pattern matching to destructure a tuple value:
    let (love, count) = tup;
    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access:
    let count = tup.1;

    // Arrays
    // Unlike a tuple, every element of an array must have the same type.
    // Arrays in Rust have a fixed length, like tuples.
    let error_code = [200, 404, 500];
    // We can access elements of an array using indexing:
    let not_found = error_code[1]; // Arrays are zero-indexed

    // You can also initialize an array to contain the same value for each element:
    let byte = [0; 8]; // This creates an array of 8 zeros

    // Functions
    // Rust code uses snake case as the conventional style for function and variable names.
    my_function();

    function_with_params(2, 7);

    // Functions can return values. There are two ways to do this in Rust:
    return_function_type_1(2, 7);
    return_function_type_2(2, 7);
}

// Function definitions in Rust start with fn and have a set of parentheses after the function name.
fn my_function() {
    println!("Another function.")
}

// The type of each parameter must be declared in the function signature.
fn function_with_params(x: i32, y: i32) {
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);
}

// Function return types are declared after an arrow (->)
fn return_function_type_1(x: i32, y: i32) -> i32 {
    let z = x + y;
    // The return keyword can be used to return a value from the function early
    return z;
}

// In Rust, the return value of the function is synonymous with the value of the final expression in the block of the function.
// You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
fn return_function_type_2(x: i32, y: i32) -> i32 {
    // This function will return the value of x + y
    // Note the lack of a semicolon at the end, making this an expression rather than a statement
    x + y
}

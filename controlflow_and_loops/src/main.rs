fn main() {
    // Call all the control flow function examples
    controlflow_type_1();
    controlflow_type_2();
    simple_loop();
    while_loop();
    for_loop();
}

// Example of if-else control flow
fn controlflow_type_1() {
    let number = 5;
    // In Rust, the condition in an if statement must be a boolean expression
    if number == 10 {
        println!("The number is equal to 10");
    } else if number < 10 {
        println!("The number is smaller than 10")
    } else {
        println!("The number is greater than 10")
    }
    // This function will print "The number is smaller than 10"
}

// Example of using if in a let statement
fn controlflow_type_2() {
    let condition = true;
    // The if expression can be used on the right side of a let statement
    // Both arms of the if must return the same type
    let number = if condition { 5 } else { 6 };
    println!("The number is: {}", number);
    // This will print "The number is: 5"
}

// Example of a simple loop with a break condition
fn simple_loop() {
    let mut counter = 0;

    // loop creates an infinite loop, which is broken by the break statement
    // The value after break is returned from the loop
    let simple_loop = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The simple_loop: {}", simple_loop);
    // This will print "The simple_loop: 10"
}

// Example of a while loop
fn while_loop() {
    let mut number = 0;
    // The while loop continues as long as the condition is true
    while number != 10 {
        println!("{}!", number);
        number += 1;
    }
    println!("LIFTOFF!!!");
    // This will print numbers from 0 to 9, followed by "LIFTOFF!!!"
}

// Example of for loops
fn for_loop() {
    let a = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    // Iterate over the elements of the array
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    // Iterate over a range of numbers
    // Note: 1..4 is a range that includes 1, 2, and 3, but not 4
    for range in 1..4 {
        println!("the range is: {}", range);
    }
    // This will print the values in the array, followed by numbers 1, 2, and 3
}

fn main() {
    // Ownership helps Rust to manage memory
    // There are three ways to manage memory
    //                -------------------------------------
    //                |   PROS         |     CONS         |
    //                |                |                  |
    //                -------------------------------------
    //   Garbage      | - Automatic    | - Performance    |
    //   Collection   |   memory mgmt  |   overhead       |
    //                | - Easier for   | - Unpredictable  |
    //                |   developers   |   pauses         |
    //                | - Prevents most| - Higher memory  |
    //                |   memory leaks |   usage          |
    //                | - Good for     | - Less control   |
    //                |   rapid dev    |   over memory    |
    //                -------------------------------------
    //   Manual       | - Fine-grained | - Prone to bugs  |
    //   Memory       |   control      | - Memory leaks   |
    //   Management   | - Predictable  | - Buffer overflows|
    //                |   performance  | - Double frees   |
    //                | - Lower memory | - Time-consuming |
    //                |   usage        | - Security risks |
    //                -------------------------------------
    //   Rust         | - Memory safety| - Steep learning |
    //   Ownership    |   at compile   |   curve          |
    //                |   time         | - Increased      |
    //                | - No runtime   |   development    |
    //                |   overhead     |   time initially |
    //                | - Predictable  | - Restrictive    |
    //                |   performance  |   borrowing rules|
    //                | - Prevents data| - Can be complex |
    //                |   races        |   for some tasks |
    //                -------------------------------------
}

//

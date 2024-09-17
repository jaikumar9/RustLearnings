fn main() {

    let mut x: i32 = 5; 
    println!("The value of x is: {}", x);
      x  = 6; 

    //  cannot assign twice to immutable variable
    println!("The value of x is: {}", x);

    println!("Hello, Variable!");
}

// *************************************************

// Constant -> const 
// const = immutable " we cannot use mut with const."

//  Constants are typically named in uppercase with underscores (e.g., THREE_HOURS_IN_SECONDS).

// *************************************************

// Shadowing in the Rust 

// fn main() {
//     let x = 5;
//     let x = x + 1; // x becomes 6
//     {
//         let x = x * 2; // In the inner scope, x becomes 12
//         println!("The value of x in the inner scope is: {x}"); // Outputs 12
//     }
//     println!("The value of x is: {x}"); // Outputs 6
// }

//Key Takeaways:
// Shadowing: Creates a new variable with the same name and allows changing the type or value.
// mut: Allows you to mutate (change) the value of a variable, but the type must remain the same.

// types change possible with shadowing not with mut.

// fn main() {
//     let spaces = "jai";  // `spaces` is a string slice (&str)
//     println!("The value of spaces is: {}", spaces);
//     let spaces = spaces.len(); // Shadowing: `spaces` is now an integer (usize)
//     println!("The value of spaces is: {}", spaces);
// }
fn main() {

let guess :u32 = "29".parse().expect("Not a number!");
println!("The value of guess is: {}", guess);
    println!("Hello, data_types!");
}


// Scalar and Compound Types
// Scalar types represent a single value and include integers, floating-point numbers, booleans, and characters.
// Compound types group multiple values into one type, like tuples and arrays.

//******************************** */

// Scalar Types 

//In Rust, scalar types are fundamental types that represent single values.

// Integer Types = Integers in Rust are numbers without fractional components. They can be:

// Signed (can hold negative and positive values) or
// Unsigned (only hold positive values).

// 1. Integer Types
// Integers in Rust are numbers without fractional components. They can be:

// Signed (can hold negative and positive values) or
// Unsigned (only hold positive values).
// Integer Size Options:
// 8-bit: i8 (signed) and u8 (unsigned)
// 16-bit: i16 and u16
// 32-bit: i32 and u32
// 64-bit: i64 and u64
// 128-bit: i128 and u128
// Architecture-dependent: isize and usize (dependent on whether you are using a 32-bit or 64-bit system)


// let x: i32 = 42;  // Signed 32-bit integer
// let y: u64 = 100; // Unsigned 64-bit integer

// Integer Overflow:
// Debug Mode: Causes a panic (runtime error) if overflow happens.
// Release Mode: Uses two's complement wrapping, meaning values wrap around (e.g., u8 value of 256 becomes 0).

// let x: u8 = 255;  // Max value for u8
// let x = x + 1;    // Overflow in release mode will wrap it to 0


//**************************************************** */
// 2. Floating-Point Types
// These types represent numbers with decimal points. Rust provides:

// f32: Single-precision (32 bits)
// f64: Double-precision (64 bits, default)
// Example:
// rust
// Copy code
// let a = 2.5;      // Default f64
// let b: f32 = 3.2; // Explicit f32

//*******************************************

// 3. Boolean Type
// Rust’s boolean type (bool) can hold two values: true or false. It’s used in logical operations and conditionals.

// Example:

// let is_true = true;        // bool
// let is_false: bool = false; // with explicit type annotation


//***************************************************** */
// compound types

//1. Tuples
// What are Tuples?

// Tuples group multiple values into one compound type.
// They can hold values of different types.
// The length of a tuple is fixed and cannot change.
// Creating a Tuple:

//  let tup: (i32, f64, u8) = (500, 6.4, 1);


// Destructuring a Tuple: 
// let (x, y, z) = tup;
// println!("The value of y is: {}", y);


// Accessing Tuple Elements:
// let five_hundred = tup.0;  // Access first element
// let six_point_four = tup.1; // Access second element
// let one = tup.2;           // Access third element


// 2. Arrays
// What are Arrays?

// Arrays are collections of multiple values of the same type.
// Arrays have a fixed size that must be known at compile time.

// Creating an Array:
// let a = [1, 2, 3, 4, 5];


// Accessing Array Elements:
// let first = a[0]; // Access first element
// let second = a[1]; // Access second element


// Invalid Access:
// Trying to access an element outside the array’s bounds will cause a runtime error (panic).
// let element = a[10]; // This will panic if the index is out of bounds


// Initializing Arrays:
// You can initialize an array with the same value for all elements:
// let a = [3; 5]; // Array with 5 elements, all set to 3

// In summary:
// Tuples: Fixed size, can hold different types, accessed via index or destructuring.
// Arrays: Fixed size, same type for all elements, accessed via index.

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }
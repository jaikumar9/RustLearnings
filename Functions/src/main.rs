use std::io;

fn main() {
    println!("Hello, Functions!");

    // another_function_without_arg();

    // three_function();

    // another_function(5);

    // print_labaled_mesurement(42,'L');
   
    // statement_and_expressions();

    // function_with_return_value();

    plus_one();
    
}


fn another_function_without_arg() {
    println!("Another function.");
}

fn three_function() {
    println!("3rd function")
}

fn another_function(x: u32) {
    println!("the value of x is {}", x);
}

fn print_labaled_mesurement(value:u32, unit_lable: char){
    print!("The measurement is: {} {}", value, unit_lable);
}

fn statement_and_expressions(){
    let y = 6;   // statement

    let x = {
        let j = 3;
        j + 6    // expression
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}


fn function_with_return_value() -> u32 {
    let result = 5 + 5;
    println!("The result is: {}", result);  // Print the value
    result  // Return the value
}

fn plus_one() -> u32 {
    let mut num_in_string = String::new();

    io::stdin()
        .read_line(&mut num_in_string)
        .expect("Failed to read line");

    // Parse the input and handle possible errors
    let num: u32 = match num_in_string.trim().parse() {
        Ok(n) => n,  // If parsing is successful, use the number
        Err(_) => {
            println!("Please enter a valid number.");
            return 0;  // Return a default value or handle the error as needed
        }
    };

    let r = num + 1;
    println!("The result is: {}", r);
    r  // Return the result
}
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number == 3 {
        println!("number was three");
    }

    // Call multiple_condition here to run it
    multiple_condition();

    // Using if in a let Statement 

    if_in_let();

    loop_example();

}

// Handling Multiple Conditions with else if
fn multiple_condition() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// Using if in a let Statement 

fn if_in_let(){

    let condition = true;

    let number = if condition {5} else {6};
    println!("The value of number and condition  is: {number} and {condition}");

}


// Repetition with Loops  

fn loop_example() {
    let mut counter = 0;

    loop {
        println!("The value of counter is {counter}");
        counter = counter + 1;

        if counter == 11 {
            break;
        }
    }
}

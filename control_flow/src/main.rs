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

    loop_return();

    loop_label();

    while_loop();

    loop_collection();

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
    //  if counter == 5 {
    //     continue;
    //  }

        if counter == 10 {
            break;
        } 
        counter = counter + 1;
    }
}

// Loop is Also an Expression 
// Returning Values from Loops

fn loop_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Loop Labels 

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}


// Conditional Loops with while 

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// Looping Through a Collection 

fn loop_collection() {
    let mut a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        a[index] += 1;
        println!("The value at index {} is: {}", index, a[index]);

        
        index += 1;
    }
}
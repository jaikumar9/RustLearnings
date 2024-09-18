fn main() {
    println!("Hello, Functions!");

    another_function_without_arg();
    three_function();
    another_function(5);
    print_labaled_mesurement(42,'L');
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
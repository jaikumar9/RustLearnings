fn main() {
     
     //Data Types

    // uint variable
    // let mut num:u8 = 100 ;
    // println!("The val stored in num is {}", num);
    // num = 1;
    // println!("The val stored in num is {}", num);
    
    // println!("Hello, Jai!");


    // &str 
    // let string_litral: &str = "Hey i'm Rust";
    // println!("The string litral is {}", string_litral);

    // String

    // let mut string_var: String = String::from("Hey i'm Rust");
    // string_var.push_str("What's up?");
    // println!("The string variable is {}", string_var);

    //Tupple 

    let emp_info: (&str,u8) = ("Jai", 23);
    
    // let emp_name = emp_info.0;
    // let emp_age = emp_info.1;

    // println!("The employee name is {} and age is {}", emp_name, emp_age);

    let (employee_name, employee_age) = emp_info;
    println!("The employee name is {} and age is {}", employee_name, employee_age);

}

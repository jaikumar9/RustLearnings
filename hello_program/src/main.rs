const GLOBAL_DATA:u8 = 121;

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

    // let emp_info: (&str,u8) = ("Jai", 23);
    
    // let emp_name = emp_info.0;
    // let emp_age = emp_info.1;

    // println!("The employee name is {} and age is {}", emp_name, emp_age);

    // let (employee_name, employee_age) = emp_info;
    // println!("The employee name is {} and age is {}", employee_name, employee_age);
     

     // functions 

    // print_val(5);
//     let num :u8 = 4;
//     let num1 :u8 = 6;
//     let sum :u8 = sum_add(num,num1);
//     println!("{}",sum)

println!("{}",GLOBAL_DATA);

// OwnerShip =>>

// this is the Stack memory 
let a = 10 ;
let b = a;
println!("{}", a);
println!("{}", b);

// This is Heap Memory  => OwnerShip will change in this 

let srt1 = String::from("jai"); // str1 is the owner of "Jai"
let srt2 = srt1;                 // Changing of the Ownership, Srt2 will be the new owner of "Jai"
// println!(" string 1{}", srt1);
println!("String 2 {}", srt2);


   
 } 
 

// fn print_val(num:u8) {
// println!("{}",num)
// }

// fn sum_add(num:u8,num1:u8) -> u8 {
//   return   num+num1;
// }


// *********************************************************************************************

// OwnerShip in Function


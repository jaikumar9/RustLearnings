// Ownership 
fn main() {

    
    // let s = "Hello ji";
    
    // {
    //     let _x = "Hello ji dobbara";
    // }

    // println!("{}", s);

    // Create a heap-allocated String
    let name = String::from("Jai");
     

     // Ownwership of num is main() due to the copy trait
    let num = 11;
    let result = add(num);

    println!("the num is {num} and result is {result}"); //But numbers mai chalata hai

     varaible_n_data();
     string_move();
     take_ownership(name);
      let s =  gives_ownership();

      let s2 =  takes_and_gives_back(s);

      let len:usize = string_length(String::from("Jai"));

    // ownership change to take_ownership() function.
    // println!("num is {name}"); 

    println!("The length of the string is {len}");

     println!("The s is {s2}");
     
}

fn varaible_n_data(){
    let mut x = 5;
    let y = x;  // number can copy by the rust
 
    x = 11;
    println!("x is {x} y is {y}");
}

fn string_move(){
    let s1 = String::from("String_Move_clone");

    // ownership change to the s2 
     let mut s2 = s1.clone();  // string can't copy 
     // clone is Expencive

     s2.push_str("yo yo");
     s2.push_str("yo yo again");
    
     println!("s1 is {s1} and s2 is {s2}");
}

 fn add( x: i32 ) -> i32 {
  x + 10
 }


 fn take_ownership(s: String) {
    println!("Inside the take ownership {s}");
 }

 fn gives_ownership() -> String {
    let s: String = String::from("A String that gives ownership");
    s
 }
 
 fn takes_and_gives_back(s2: String) -> String {
    println!("Inside the takes_and_gives_back {s2}");
    s2
 }

 fn string_length(s: String) -> usize {
    s.len()
 }
 
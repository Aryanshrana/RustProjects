use std::io;
fn main() {
    println!("Hello, world!");
    //expression evaluae to a value and statements does not return a value
    //expressions does not have a ending semicolon if we add we will making it statements
    another_function("Aryansh");
    another_function2(12, 'm');


    //expression example
    let y = {
        let x = 76;
        x+1
    };
    println!("The value of y is {y}");
    let z = plus_value(100,y);
    println!("The value of new z is {z}");

    // we have understood expressions and functions ; here we are introducing control flow with another function
    control_flow();

}
fn another_function(x: &str) {
    println!("Hello, again {}",x);
}
fn another_function2(x: i32, c: char){
    println!("The value you passes is {x} {c}");
}
fn plus_value(z:i32,y:i32) -> i32 {
    println!("The value of z is {z}");
    z+y// -> it is expression so it does not have ending semicolon
}
// CONTROL FLOW 
fn control_flow() {
    let mut number = String::new();
    println!("enter a number");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");
    let number2: u32 = number.trim().parse().expect("invalid");
    
    if number2 % 4 == 0 {
        println!("Hello Aryansh");
    }
    else {
        println!("Hello Shubham");
    }

}


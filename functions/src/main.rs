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
    control_flow2();

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

    //using let and if
    let mut decision = String::new();
    println!("Enter decision as true or false");
    io::stdin()
        .read_line(&mut decision)
        .expect("failed to read input");
    
    let mut condition: bool;
    if decision == "true" {
        condition = true;
    }
    else if decision == "false"{
        condition = false;
    }
    else {
        println!("error occured in taking decision");
        return;
    }
    let value = if condition {5} else {7};

    println!("The value we get is {value}");
    
}
fn control_flow2() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 12 {
            break counter * 2;
        }
    };
    println!("the value we get of result is:  {result}");
    // looping through a collection
    let a: [i32;5] = [10,20,30,40,50];
    let mut count = 0;
    while count < 5 {
        println!("the value of index {count} is: {}",a[count]);
        count+=1;
    }
    //using for loop
    for number in (1..4).rev() { // rev means it takse values in reverse order 
        println!("{number}!");
    }
    println!("TakeOff!!!");
}

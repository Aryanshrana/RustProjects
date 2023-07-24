fn main() {
    println!("Hello, world!");
    another_function("Aryansh");
    another_function2(12, 'm');

    //expression example
    let y = {
        let x = 76;
        x+1
    };
    println!("The value of y is {y}");
    let z = plus_value(100,y);
    println!("The value of new z is {z}")
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
//expression evaluae to a value and statements does not return a value
//expressions does not have a ending semicolon if we add we will making it statements

fn main() {
    //let s = "hello";//string literal stored in stack
    //another data type String type stored in heap
    let mut s = String::from("hello");//converting string literal to String type
    //here we are storing it in heap; so String:from is asking for memory at run time
    s.push_str(",world!");//push_str append string literal to String
    println!("{}",s);

    //to give back the memory when we don't use it
    //When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

    //Scenarios where owership concept works:
    
}

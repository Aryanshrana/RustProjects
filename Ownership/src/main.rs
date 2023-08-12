fn main() {
    //let s = "hello";//string literal stored in stack
    //another data type String type stored in heap
    
    //-->let mut s1 = String::from("hello");//converting string literal to String type
    
    //here we are storing it in heap; so String:from is asking for memory at run time
    
    //-->s1.push_str(",world!");//push_str append string literal to String
    //-->println!("{}",s1);

    //to give back the memory when we don't use it
    //When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

    //Scenarios where owership concept works:

    //vatiables and data interacting with move
    
    //-->let mut s2 = s1;

    //here what ponter , length, capacity of s1 is copied to s2 but actual data in heap is same so both s2 and s1 are copied to same data in heap memory which will give us double free memory error if rust try to free the error using drop function twice, which will give us security vulnerabilities, to avoid this what rust do it makes s1 invalid and we say s1 is moved into s2.
    //we do not deeply copy the data in heap because that will be inexpensive in runtime if data is too big
    
    //-->s2.push_str(",backstage!");
    //-->println!("{}",s2);

    // variables and data interacting with clone 
    // is we want to do deep copy of the data we can use clone function
    //-->let mut s3 = s1.clone();
    //-->println!("s1={},s3={}",s1,s3);

    let s3 = String::from("Bonjour!"); // s3 comes under scope
    takes_ownership(s3); // we call a function here s3 moves and no longer valid

    let x = 32;//x comes in scope
    makes_copy(x); //x is copied to another variable and not moved so x is still valid after this
    println!("{}",x+2);
}
//here x goes out of scope and the s3 but is moved so nothing special happens 
fn takes_ownership(some_string: String){ // some_string is comes into scope
    println!("some_string = {}",some_string);
} // some_string goes out of the scope and drop function is called and is no longer valid
fn makes_copy(some_value:i32){ // some_value comes in scope
    println!("some_value = {}",some_value+3);
} // some_value goes out of scope
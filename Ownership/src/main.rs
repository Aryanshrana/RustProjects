fn main() {
    //let s = "hello";//string literal stored in stack
    //another data type String type stored in heap
    let mut s1 = String::from("hello");//converting string literal to String type
    //here we are storing it in heap; so String:from is asking for memory at run time
    s1.push_str(",world!");//push_str append string literal to String
    println!("{}",s1);

    //to give back the memory when we don't use it
    //When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

    //Scenarios where owership concept works:

    //vatiables and data interacting with move
    let mut s2 = s1;
    //here what ponter , length, capacity of s1 is copied to s2 but actual data in heap is same so both s2 and s1 are copied to same data in heap memory which will give us double free memory error if rust try to free the error using drop function twice, which will give us security vulnerabilities, to avoid this what rust do it makes s1 invalid and we say s1 is moved into s2.
    //we do not deeply copy the data in heap because that will be inexpensive in runtime if data is too big
    s2.push_str(",backstage!");
    println!("{}",s2);

    

}

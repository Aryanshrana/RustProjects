fn main() {
    // here we are seeing how ownership is changing for a value as it passes to a function or return to a calling function
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("s1 = {}",s1);
    let s2 = String::from("welcome"); //s2 comes in scope
    let s3 = takes_and_gives_back(s2); // s2 moves into takes_and_gives_back which also moves it'sreturn value to s3
    println!("s3 = {}",s3);
} // s3 goes out of scope and drop is called , s2 was moved so nothing happens , and s1 goes out of scope so nothing happened
fn gives_ownership() -> String {

   let some_string = String::from("you are"); //some_string comes in scope
   some_string //some_string is returned and moves to function who is calling it

}
fn takes_and_gives_back(mut my_string: String) -> String {

    my_string.push_str(" to our house");
    my_string //a_string is returned and moves out to calling function
}

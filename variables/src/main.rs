fn main() {
    //explaing concepts of variables and their mutability
    let mut x = 5;
    println!("the value of x is {x}");
    x = 78;
    println!("the value of x is {x}");
    //you can define constants in this way
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;

    //shadowing -> is varaibles define with same name again they overwrite previous version.
   
    {
        //inner shadowing
        let x = x*2;
        println!("the value of x is {x}");
    }
    let x = x + 1;
    println!("new value of x is {x}");

    //another mutability example
    let spaces = "   ";
    let spaces = spaces.len();
    println!("new value of spaces is {spaces}");


}

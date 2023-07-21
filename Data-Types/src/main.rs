use std::io;
fn main() {
    println!("Hello, world!");
    //Scalar Datatypes
        //Integer
    let x: u32 = 45;
        //floating point
    let y: f64 = 23.0;
        //character
    let c: char = 'Z';
        //boolean
    let f: bool = true;//(1 byte)
    
    // compound DataTypes
        //tuple
    let tup: (i32, f64, char) = (-234,56.4,'S');
    let (x,y,z) = tup;
    println!("The value of tuple's elements are: {x},{y},{z}");
            //we can also do this for defining tuple's element
    //let fifty_six_point_four = tup.1;
    //let minus_two_hundred_thirty_four = tup.0;
    //let S = tup.2;
            //The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type.
        //arrays
            //they have fixed length like tuple but also have same type like tuple
            //defining array
    let a: [i32;5] = [12,13,14,15,16];
    //let b = [2;4];  // => same as b: [i32,4] = [2,2,2,2] 
    //let first = a[0];
    let third = a[2];

    println!("The third element of array a is {third} ");

    // accessing array elements
    
    let arr = [100,110,120,130,140];

    println!("please input the index to access elements");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("can not read input");
    
    let index: usize = index.trim().parse().expect("enter index as number");

    let element = arr[index];
    println!("The element you accessed is {element}");




}

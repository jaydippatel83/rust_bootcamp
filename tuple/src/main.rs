#![allow(unused)]

fn main() { 

    let tup: (i32, f64, u8, char) = (500, 6.4, 1, 'x');
    let (x, y, z, a) = tup;
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of a is: {a}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = x;
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");

 
    //nested tuple
    let x: ((i32, f64), u8) = ((500, 6.4), 1);
    let ((a, b), c) = x;
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");

    //tuple with one element        
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = x;
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");

    //tuple with one element
    let x: (i32, f64, u8) = (500, 6.4, 1);          
    let (a, b, c) = x;
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");

    //tuple with one element
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = x;  
    println!("The value of a is: {a}");
    println!("The value of b is: {b}");
    println!("The value of c is: {c}");

    //tuple with one element
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = x;  
    println!("The value of a is: {a}");
}

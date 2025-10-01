#![allow(unused)]


const NUM: u32 = 10;

fn main() {
    //variables
    println!("Hello, world!");

    let mut x = 5;
    x += 10;
    println!("The value of x is: {x}");

    let y = -1;
    println!("The value of y is: {y}");
 
    let x: i32 =1;
    let x: i32 =  2;
    let x: bool = true;

    let x= 1;
    let z= x + x;
    println!("The value of x is: {x} + {x} = {z}");
 
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    //data types
    let x: i32 = 1;
    let y: i32 = 2;
    let z: i32 = x + y;
    println!("The value of z is: {z}");

    println!("x: {:?}", x);
    println!("y{:#?}", y);
    println!("z{:#?}", z);
 

    
}   


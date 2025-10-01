#![allow(unused)]

fn main() { 
    //data type

    // signed integer
    let x: i8 = 127;
    let y: i16 = 32767;
    let z: i32 = 2147483647;
    let a: i64 = 9223372036854775807;
    

    // unsigned integer
    let x: u8 = 255;
    let y: u16 = 65535;
    let z: u32 = 4294967295;
    let a: u64 = 18446744073709551615;
    

    // floating point
    let x: f32 = 3.14;
    let y: f64 = 3.141592653589793;
    
    // boolean
    let x: bool = true;
    let y: bool = false;


    // character
    let x: char = 'x';
    let y: char = 'y';
    let z: char = 'z';

    // string
    let x: String = String::from("Hello");
    let y: String = String::from("World");


    let mut x= u32::MAX;
    println!("x: {}", x);
    x= x + 1;
    println!("x: {}", x);


    let x= u32::checked_add(u32::MAX, 1);
    println!("x: {:?}", x);

    let x= u32::checked_sub(u32::MIN, 1);
    println!("x: {:?}", x); 

    let x= u32::checked_mul(5, 2);
    println!("x: {:?}", x);
    let x= u32::checked_div(10, 2);
    println!("x: {:?}", x);

    let x= u32::checked_rem(10, 3);
    println!("x: {:?}", x);


    let x= u32::wrapping_add(u32::MAX, 1);
    println!("x: {:?}", x);

    let x= u32::wrapping_sub(u32::MIN, 1);
    println!("x: {:?}", x);

    let x= u32::wrapping_mul(5, 2);
    println!("x: {:?}", x);


    let x= u32::wrapping_div(10, 2);
    println!("x: {:?}", x);

    let x= u32::wrapping_rem(10, 3);
    println!("x: {:?}", x);


}

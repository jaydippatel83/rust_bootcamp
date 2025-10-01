#![allow(unused)]

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn print_value(x: String) {
    println!("The value of x is: {x} {x} {x} {x}");
}

fn sub(x: i32, y: i32) -> i32 {
    x - y
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}

fn div(x: i32, y: i32) -> i32 {
    x / y
}

fn mode(x: i32, y: i32) -> i32 {
    x % y
} 
 

fn main() {
    let x = 10;
    let y = 20;
    let z = add(x, y);
    println!("The value of z is: {z}");
    let x = String::from("Hello");
    print_value("(`_`)".to_string());

    let x = 10;
    let y = 20;
    let z = sub(x, y);
    println!("The value of z is: {z}");

    let x = 10;
    let y = 20;
    let z = mul(x, y);
    println!("The value of z is: {z}");

    let x = 10;
    let y = 20;
    let z = div(x, y);
    println!("The value of z is: {z}");

    let x = 10;
    let y = 20;
    let z = mode(x, y);
    println!("The value of z is: {z}"); 
  
}

#![allow(unused)]

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn print_value(x: String) {
    println!("The value of x is: {x} {x} {x} {x}");
}

fn main() {
    let x = 10;
    let y = 20;
    let z = add(x, y);
    println!("The value of z is: {z}");
    let x = String::from("Hello");
    print_value("(`_`)".to_string());
    
}

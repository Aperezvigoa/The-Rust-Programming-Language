// Variables in Rust are immutable by default.

fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    // Cannot assign twice to immutable variable
    // x = 6;
    // println!("The new value of x is: {x}");

    // We can declare a mutable variable using mut
    let mut y = 5;
    println!("The value of y is: {y}");

    y = 6;
    println!("The new value of y is: {y}");
}
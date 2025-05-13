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

    // Like immutable variables, constants are not allowed to change
    // You are not allowed to use mut with constants
    // You declare const using the const keyword and the type of the value must be annotated
    // Constant may be set only to a constant expression, not the result of a value
    // Example:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("This is a const example: {THREE_HOURS_IN_SECONDS}") // 10800

}
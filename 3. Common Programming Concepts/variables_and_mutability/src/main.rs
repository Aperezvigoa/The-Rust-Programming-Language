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

    // Constants
    // Like immutable variables, constants are not allowed to change
    // You are not allowed to use mut with constants
    // You declare const using the const keyword and the type of the value must be annotated
    // Constant may be set only to a constant expression, not the result of a value
    // Example:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("This is a const example: {THREE_HOURS_IN_SECONDS}"); // 10800


    // Shadowing
    // You can declare a new variable with the same name as a previous variable
    // So the first variable is shadowed by the second, which means that the second variable is what the compiler will see
    // We can shadow a variable by using the same variable's name and repeating the use of the let keyword

    let value = 10;
    let value = value + 1;
    
    // Creation of inner scope
    {
        let value = value + 50;
        println!("The value of 'value' in the inner scope is: {value}");
    }

    println!("The value of 'value' is: {value}");
    // One of the differences between mutable value and shadowing is that we are creating a new value, and we can assign a new type
    // Example:
    let spaces = "    ";
    let spaces = spaces.len();

    println!("spaces: {spaces}")
}
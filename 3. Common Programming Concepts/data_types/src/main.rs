use core::panic;
use std::io;

fn main() {

    // --- Integer types

    // Declaring a 8 bit unsigned integer.
    let eight_bit_uinteger: u8 = 255;
    println!("A 8-bit unsigned integer: {eight_bit_uinteger}.");

    // Declaring a 32 bit signed integer.
    let tt_bit_integer: i32 = -1492;
    println!("A 32-bit signed integer: {tt_bit_integer}.");

    // Using a signed architecture-depends size integer
    let arch_integer: isize = 10000000000;
    println!("I'm a 64 bits signed integer, because my architecture is 64 bits: {arch_integer}.");

    // --- Using Literals

    // Hex
    let hex_num: i32 = 0xff; // 255
    println!("Example of hexadecimal number: {hex_num}.");

    // Octal
    let octal_num: i32 = 0o65; // 53
    println!("Example of octal number: {octal_num}.");

    // Binary
    let bin_num: i32 = 0b1010_1101; // 173
    println!("Example of binary number: {bin_num}.");

    // --- Floating-Point Types

    let default_fpoint = 2.0; // f64 as default
    println!("The default type of floating-point types is f64: {default_fpoint}.");

    // Declaring a f32 type
    let f32_fp: f32 = 3.56;
    println!("Example of f32 type: {f32_fp}.");

    // --- Numeric Operations

    // Addition
    let sum: i32 = 16 + 42;
    println!("The addition result is {sum}.");

    // Substraction
    let difference: f64 = 95.5 - 4.3;
    println!("The substraction result is {difference}.");

    // Multiplication
    let product: i32 = 4 * 30;
    println!("The multiplication result is {product}.");

    // Division
    let quotient: f64 = 56.7 / 32.2;
    println!("The quotient division between two floating point types is {quotient}.");
    let truncated: i32 = -29 / 8;
    println!("The truncated result between two integers is {truncated}.");

    // remainder
    let remainder = 46 % 5;
    println!("The modulus is {remainder}.");

    // --- Boolean type

    let t: bool = true;
    let f: bool = false;
    println!("The result of first boolean is {0} and the second is {1}.", t, f);

    // --- Char Type

    let cute_cat_emoji: char = '🐱';
    let letter_z_char: char = 'z';
    println!("Example of chars: {cute_cat_emoji} & {letter_z_char}");


    // --- Compound Types

    // --- Tuple
    let tup: (i32, f64, bool) = (500, 3.14, true);

    let (ivar, fvar, bvar) = tup;
    println!("Value of ivar: {ivar}, value of fvar: {fvar}, value of bvar: {bvar}.");

    // Other way to access tuple values
    let new_tup: (f64, bool, i32) = (3.14, true, 201019);

    let pi: f64 = new_tup.0;
    let im_handsome: bool = new_tup.1;
    let my_cat_birth: i32 = new_tup.2;

    println!("Pi: {pi}\nI'm handsome: {im_handsome}\nMy cat birth is: {my_cat_birth}");

    // Creating a mutable tuple
    let mut mutable_tup: (i32, i32) = (0, 0);
    mutable_tup.0 = 2001;
    mutable_tup.1 = 2019;

    let x: i32 = mutable_tup.0;
    let y: i32 = mutable_tup.1;
    println!("x = {x}\ny = {y}");

    // --- Array

    #[allow(unused_variables)]
    let first_arr: [i32;5] = [1, 2, 3, 4, 5];
    #[allow(unused_variables)]
    let months = ["January", "Febraury", "March", "April", "May"];
    #[allow(unused_variables)]
    let arr_2d2 = [3, 5]; // [3, 3, 3, 3, 3];

    // Accessing array elements
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    // Reading line and saving into index.
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // Shadowing index to convert it into usize.

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!("The index entered is: {index}");

    let element: i32 = arr[index];

    println!("The arr element at index {index} is: {element}");
}
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

}
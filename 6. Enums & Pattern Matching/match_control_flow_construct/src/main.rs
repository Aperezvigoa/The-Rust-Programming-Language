// The match Control Flow Construct
// Rust has an extremely powerful control flow construct called match, that allows you to compare
// a value against a series of patterns and the execute code based on which pattern matches.
// Patterns can be made up of literal values, variable names, wildcards and more.
// The power of match comes from the expressiveness of the pattern and the fact that the compiler
// confirms that all posible cases are handled.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}

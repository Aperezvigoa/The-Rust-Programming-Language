// The match Control Flow Construct
// Rust has an extremely powerful control flow construct called match, that allows you to compare
// a value against a series of patterns and the execute code based on which pattern matches.
// Patterns can be made up of literal values, variable names, wildcards and more.
// The power of match comes from the expressiveness of the pattern and the fact that the compiler
// confirms that all posible cases are handled.

// Patterns that Bind to Values
// A useful feature of march arms is that they can bind to the parts of the values that match the
// pattern.
// As an example, let's change one of our enum variants to hold data inside it. From 1999 though
// 2008, USA minted quarters with different design for each of the 50 states in ine side.
// No other coins got state designs, so only quarters have this extra value. We can add this info
// to out enum by changing quarter variant to include a UsState value stored.

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Now in the match expression for this code, we add a variable called state to the pattern that
// matches values of the variant. When a Coin::Quarter matches, the state variable will bind to the
// value of that quarter's state.

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

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

// Matching with Option<T>
// In the previous section, we wanted to get the inner T value of Some case when using Option<T>;
// we can also handle Option<T> using match, as we did with the Coin enum. Instead of comparing
// coins, we'll compare tje variants of Option<T>.
// Let's write a function that takes Option<i32> and, if there's a value inside, adds 1 to the
// value. If there is not a value inside, the function should return the None value and not
// attemp to perform any operations.
// Combining match and enums is useful, we'll see this pattern a lot in Rust: match against
// an enum, bind a variable to the data inside, and then execute code based on it.

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Matches are Exhaustive
// There's one other aspect of match to cover, the arms patterns must cover all possibilities.
// Using enums, we can also take special actions for a few particular values, but for all other
// values take one default action. Imagine we're implementing a game where, if you roll a 3 on a
// dice roll, your doesn't move, but instead gets a new fancy heat. If you roll a 7, your player
// loses a fancy hat. For all other values, the player moves.

fn roll_dice(dice: u8) {
    match dice {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(dice),
    }
}

fn add_fancy_hat() {
    println!("Player gets a new fancy hat");
}

fn remove_fancy_hat() {
    println!("Player lose the fancy hat");
}

fn move_player(num_spaces: u8) {
    println!("Player moves {num_spaces} spaces");
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

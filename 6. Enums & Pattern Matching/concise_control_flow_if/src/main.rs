// Concise Control Flow with let if - let else
// The if let syntax lets you combine if and let into a less verbose way to handle values
// that match one pattern while ignoring the rest.

fn main() {
    let config_max: Option<u8> = Some(3);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // This syntax if let, takes a pattern and an expression separated by an equal sign.
    // It works the same way as a match, where the expression is given to the match and
    // the pattern is its first arm. In this case, the pattern is Some(max), and the max
    // binds to the value inside the Some. We can then use max in the body of the if let
    // block in the same way we would use max in a match.
    // Using if let means less typing, less indentation and less boilerplate code.
    // However, you lose the exhaustive cheking match enforces that ensure you are not
    // forgetting to handle any cases. Choosing between match and if let depends on what
    // you're doing in your particular situation.
    // We can include an else with an if let. The block of code tht goes with the else is
    // the same as the block of code that would go with the _ case in the match
    // expression that is equivalent to the if let and else.
    // Lets create a function that takes a Coin enum, and counts all coins that are no a
    // quarter

    let mut count = 0;
    let coin = Coin::Penny;
    if let Coin::Quarter(city) = coin {
        println!("Quarter from {city:?}");
    } else {
        count += 1;
    }
}

#[derive(Debug)]
enum City {
    Madrid,
    Barcelona,
    Valencia,
    Cantabria,
    Leon,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(City),
}

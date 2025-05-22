fn main() {
    
    // --- The if statement

    let some_condition: bool = 15 > 10;
    
    if some_condition {
        println!("Yeah, it's true");
    }

    if !some_condition {
        println!("Yeah, it`s false");
    }

    // --- if & else if & else
    println!("\n*** If & else if & else ***\n");

    let seasson: &str = "Winter";

    if seasson == "Winter" {
        println!(r"It's christmas! Let's take a hot chocolate :)");
    } else if seasson == "Autumn" {
        println!("The streets are orange!");
    } else if seasson == "Spring" {
        println!("Wow! There are a lot of flowers!");
    } else if seasson == "Summer" {
        println!("Hey! Let's go to the beach!");
    } else {
        println!("mmm, I can't recognize that seasson...");
    }

    // --- Assigning Result of if Statement to Variable

    let user_points: i32 = 12_589;
    let user_league: &str = {
        if user_points < 1000 {
            "bronze"
        } else if user_points < 5000 {
            "silver"
        } else if user_points < 10_000 {
            "gold"
        } else {
            "platinum"
        }
    };

    println!("The user league is: {user_league} with {user_points} pts");

    println!("The number 2 is {}", even_or_odd(&2));

    // --- The match Statement
    println!("\n*** The Match Statement ***\n");

    let some_condition_evaluation: bool = true; // Hardcoded example

    match some_condition_evaluation {
        true => {
            println!("Some code execution because the evaluation is true.");
        }
        false => {
            println!("Another code execution because the evaluation is false.");
        }
    }

    // --- Returning from match statement & using underscore in a Match Arm

    let actual_age: i32 = 23;

    let life_state: &str =  match actual_age {
        0..= 3 => "Baby",
        4..=12 => "Kid",
        13..=17 => "Teenager",
        18..=64 => "Adult",
        65..=120 => "Senior",
        _ => "The real Mathussalen",
    };

    println!("You are {actual_age} years old, so you are an {life_state}");

    // Refactoring seassons else if block with match statement

    let seasson: &str = "Winter";

    match seasson {
        "Summer" => println!("It's time to go to the beach and swimming with white sharks :P"),
        "Spring" => println!("It's time to go shopping and walk around central park"),
        "Autumn" => println!("It's time to wear a nice yellow pair of boots"),
        "Winter" => println!("It's time to go sleep, because Santa is comming to town!"),
        _ => println!("I cannot recognize the seasson!"), // _ value MUST be the last one!!!!
    }
    
    // Match Statement With Multiple Values And Conditions

    let number: i32 = 8;

    match number {
        2 | 4 | 6 | 8 => println!("The number {number} is even."),
        1 | 3 | 5 | 9 => println!("The number {number} is odd"),
        _ => println!("The number should be between 1..9"),
    }

    match number {
        // We can think in 'value' as a internal scope variable of match
        value if value % 2 == 0 => println!("The number {value} is even."),
        value if value % 2 != 0 => println!("The number {value} is odd"),
        _ => println!("Your computer explodes!"),
    }

    // --- Loops
    println!("\n*** Loops ***\n");

    let some_text: &str = "I love Rust";
    let mut iterations: i32 = 0;

    loop {
        if iterations >= 5 {
            break;
        }
        iterations += 1;

        if iterations == 3 {
            continue;
        }
        println!("{}.- {}", iterations, some_text);
    }

    println!("");

    iterations = 0;
    
    while iterations < 5 {
        iterations += 1;

        if iterations == 3 {
            continue;
        }

        println!("{}.- {}", iterations, some_text);
    }

    // --- Recursion
    println!("\n*** Recursion ***\n");

    recursive_count_down(10);
}

// --- Assignin Result of if Statement to Variable


fn even_or_odd(number: &i32) -> &str {
    let result: &str = if number % 2 == 0 {
        "Even"
    } else {
        "Odd"
    };
    result
}

// --- Recursive function example

fn recursive_count_down(mut count_start: i32) {
    println!("Count state {count_start}");
    count_start -= 1;

    if count_start > 0 {
        recursive_count_down(count_start);
    } else {
        println!("We have finished here!");
    }
}

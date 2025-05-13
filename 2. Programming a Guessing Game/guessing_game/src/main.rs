
// To obtain the user input, we need to bring the io <input/output> library into scope.
// The io library comes from standard library, known as std:
use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {

        println!("Please input your guess.");

        // Creation a variable to store the user input using let
        // We use mut to indicate that this variable is mutable
        // We can have immutable values typing let apples = 5;
        // In Rust, variables are immutable by default.
        // String::new() creates a new empty string
        let mut guess = String::new();

        // Here we call the stdin function from the io module
        io::stdin()
            .read_line(&mut guess) // calls the read_line method to get the input from the user
            .expect("Failed to read line"); // Handle the error if Result (from read_line) is an Err

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}"); // {} is a placeholder for guess

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
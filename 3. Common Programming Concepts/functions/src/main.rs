fn main() {
    // --- Using Functions
    function_example();
    printing_argument(5);
    print_user_info("Alex", 23, true);

    // Expression example:

    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };
    println!("Value of y: {y}");

    // Using Return Values From Functions
    let five_val: i32 = five();
    println!("Five_val = {five_val}");

    let sample_value: i32 = 21;
    println!(
        "The actual value of \"sample_value\" is {sample_value}, lets add one using a function: {}",
        add_one_to_val(sample_value)
    );
}

// Basic Function
fn function_example() {
    println!("Hey, I'm a basic print function!");
}

// Function With Parameters.
fn printing_argument(x: i32) {
    println!("Value of x is: {x}");
}

// Function With Multiple Parameters
fn print_user_info(name: &str, age: i32, is_working: bool) {
    println!("{name} is {age} years old. {name} is working? {is_working}");
}

// Function With Return Value
fn five() -> i32 {
    5
}

fn add_one_to_val(x: i32) -> i32 {
    x
}

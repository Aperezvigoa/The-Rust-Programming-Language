fn main() {

    // --- If Expressions
    
    let age: i32 = 23;

    if age < 18 {
        println!("You cannot access here, this is only for adults.");
    } else {
        println!("Hey welcome to the irish bar");
    }

    let number: i32 = 3;

    if number != 0 {
        println!("Number is something other than zero.");
    }

    // --- Handling Multiple Conditions With else if
    
    let number: i32 = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("{} is not divisible by 4, 3 or 2", number);
    }

    // --- Using if in a let Statement

    let condition: bool = true;
    let number: i32 = if condition {5} else {0};

    println!("Number value is: {number}");

    // --- Loops

    /* 
        loop {
            println!("Hey, I'm infinite!");
        }
    */

    // Returning values from loop & using break keyword
    let mut counter: i32 = 5;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // We return the value after using the break keyword
        }
    };
    println!("The result is {result}");

    // Loop lables & Nested loops

    println!("\n---- Nested Loops ----\n");
    let mut count: i32 = 0;

    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");

            if remaining == 9 {
                break ;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count: {count}");


    println!("\n---- PRACTICAL EXAMPLE ----\n");

    let main_dev: (&str, i32, f64) = ("Alejandro", 23, 1500.56);
    let web_dev: (&str, i32, f64) = ("Jose", 22, 1229.30);
    let legal_dude: (&str, i32, f64) = ("Carlos", 22, 1218.30);

    let developer_team: [(&str, i32, f64); 3] = [main_dev, web_dev, legal_dude];

    'outer_loop: loop {

        println!("Yes I'm SIMP");
        let mut index: usize = 0;

        loop {
            if index == developer_team.len() {
                break 'outer_loop;
            }
            println!("\n            *********************************");
            println!(r"
            DEVELOPER NAME -> {}
            DEVELOPER AGE -> {}
            DEVELOPER SALARY -> {}", developer_team[index].0, developer_team[index].1, developer_team[index].2);
            index += 1;
        }
    }

    // While loop
    println!("\n---- While loop ----\n");

    let mut number: i32 = 3;

    while number != 0 {

        println!("{number}");
        number -= 1;
    }
    println!("LISTOFF!!");

    // For Loop
    println!("\n---- For loop ----\n");

    let names: [&str; 5] = ["Alex", "Rachel", "Cheddar", "Whiskers","Lily"];
    
    for name in names {
        println!("name: {name}");
    }

    for numbers in {0..=5}.rev() {
        println!("{numbers}")
    }
    println!("FINISHED!");
}
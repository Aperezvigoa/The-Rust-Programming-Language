use std::io;
fn main() {

    let mut inventory: [(i32, String, i32, f64); 10] = fill_toy_inventory();

    'program: loop {
        
        print_options();

        let mut user_choice: String = String::new(); 
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Error reading input");

        match user_choice.trim() {

            "1" => {
                {
                    let id: usize = request_id();
                    delete_a_toy(id, &mut inventory);
                }
            }
            "2" => {
                show_inventory(&inventory);
            }
            "3" => {
                let id: usize = request_id();
                let prize: f64 = request_new_prize();

                change_prize(id, prize, &mut inventory)
            }
            "4" => {
                short_by_prize(&inventory);
            }
            "5" => {
                break 'program;
            }
            _ => {
                println!("Invalid option...");
                continue 'program;
            }
        }
    }
}

fn print_options() {
    println!("\n1. Remove a toy");
    println!("2. Show inventory");
    println!("3. Edit prize");
    println!("4. Short by prize");
    println!("5. Exit program\n");
}

fn short_by_prize(inventory: &[(i32, String, i32, f64); 10]) {

    let mut shorted_inventory = inventory.clone();
    
    for _i in 0..shorted_inventory.len(){

        for j in 0..shorted_inventory.len() - 1 {

            if shorted_inventory[j].3 > shorted_inventory[j + 1].3 {
                let temp = shorted_inventory[j].clone();
                shorted_inventory[j] = shorted_inventory[j + 1].clone();
                shorted_inventory[j + 1] = temp;
            }
        }
    }

    println!("Printing shorted inventory:");
    show_inventory(&shorted_inventory);
}

fn request_new_prize() -> f64 {
    println!("Enter the new prize:");
    let mut user_id: String = String::new();

    io::stdin()
        .read_line(&mut user_id)
        .expect("Something went wrong!");

    user_id.trim().parse().expect("Cannot convert to f64.")
}

fn request_id() -> usize {
    println!("Enter the toy ID:");
    let mut user_id: String = String::new();

    io::stdin()
        .read_line(&mut user_id)
        .expect("Something went wrong!");
    user_id.trim().parse().expect("Cannot convert to usize.")
}

fn change_prize(id: usize, prize: f64, inventory: &mut [(i32, String, i32, f64); 10]) {

    if (id < inventory.len()) && prize > 0.0 {

        inventory[id].3 = prize;
        println!("Prize updated!");
    } else {
        println!("Something went wrong!");
    }
}

fn delete_a_toy(id: usize, inventory: &mut [(i32, String, i32, f64); 10]) {

    if id < inventory.len() {
        inventory[id].1.clear();
        inventory[id].2 = 0;
        inventory[id].3 = 0.0;

        println!("Toy deleted successfully!");
    } else {
        println!("Be careful! The ID is not recognized!");
    }
} 

fn show_inventory(inventory: &[(i32, String, i32, f64); 10]) {

    for i in inventory {
        println!("* * * * * * * * * ID: {} * * * * * * * * *", i.0);
        println!("Toy Name: {}", i.1);
        println!("Toy Quantity: {}", i.2);
        println!("Toy Prize: {}", i.3);
    }
}

fn fill_toy_inventory() -> [(i32, String, i32, f64); 10] {

    let mut toy_inventory: [(i32, String, i32, f64); 10] =
        std::array::from_fn(|_| (0, String::new(), 0, 0.0));

    for i  in 0..10 {
        println!("Register the toy nº {}", i + 1);
        toy_inventory[i] = create_toy(i as i32);
        println!();
    }
    
    toy_inventory    
}

fn create_toy(id: i32) -> (i32, String, i32, f64) {

    let mut input: String = String::new();
    let mut toy: (i32, String, i32, f64) = (id, String::new(), 0, 0.0);

    toy.0 = id;
    
    println!("Enter the name of the toy");
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong!");

    toy.1 = input.trim().to_string();

    println!("Enter the quantity of {}", toy.1);
    input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong!");

    toy.2 = input.trim().parse().expect("Expected receiving a integer!");    
    
    println!("Enter the price of one {}", toy.1);
    input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong!");

    toy.3 = input.trim().parse().expect("Expected receiving a float");

    toy
}

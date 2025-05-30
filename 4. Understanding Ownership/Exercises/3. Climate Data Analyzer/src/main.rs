use std::io;

struct Register {
    day: u32,
    temperature: f64,
    seasson: String,
}

fn main() {
    println!("Welcome to the temperature register analyzer.");
    let mut data: Vec<Register> = Vec::new();
    'program: loop {
        print_menu();
        println!("-------------------");
        let mut choice: String = String::new();
        read_input(&mut choice);

        match choice.trim() {
            "1" => {
                println!();
                data.push(new_register());
                println!();
            }
            "2" => {
                if !is_data_empty(&data) {
                    println!();
                    let results = process_registers(&data);
                    println!("ANALYSIS RESULT");
                    println!("Average:         {}ºC", results.0);
                    println!("Max temperature: {}ºC", results.1);
                    println!("Min day:         {}", results.2);
                    println!();
                }
            }
            "3" => {
                if !is_data_empty(&data) {
                    let selected_seasson = select_seasson();
                    let filtered_data = filter_by_seasson(selected_seasson, &data);
                    if filtered_data.len() > 0 {
                        for i in filtered_data {
                            println!("Day:  {}", i.day);
                            println!("Temp: {}ºC", i.temperature);
                            println!("------------");
                        }
                    } else {
                        println!("No data to analyze...");
                    }
                }
            }
            "4" => {
                if !is_data_empty(&data) {
                    let winter = String::from("Winter");
                    let spring = String::from("Spring");
                    let summer = String::from("Summer");
                    let autumn = String::from("Autumn");
                    let seassons: Vec<String> = vec![winter, spring, summer, autumn];

                    let winter_data: Vec<Register> =
                        filter_by_seasson(seassons[0].to_string(), &data);
                    let spring_data: Vec<Register> =
                        filter_by_seasson(seassons[1].to_string(), &data);
                    let summer_data: Vec<Register> =
                        filter_by_seasson(seassons[2].to_string(), &data);
                    let autumn_data: Vec<Register> =
                        filter_by_seasson(seassons[3].to_string(), &data);

                    let splitted_data: Vec<Vec<Register>> =
                        vec![winter_data, spring_data, summer_data, autumn_data];

                    let mut index: usize = 0;
                    for i in splitted_data {
                        if enough_data(&i, seassons[index].to_string()) {
                            detect_anomaly(seassons[index].to_string(), &i);
                        }
                        index += 1;
                    }
                }
            }
            "5" => {
                println!("Good bye!");
                break 'program;
            }
            _ => {
                println!("Invalid choice.");
                continue 'program;
            }
        }
    }
}

fn enough_data(data: &Vec<Register>, seasson: String) -> bool {
    if data.len() < 2 {
        println!("Not enough data from {} to detect anomalies.", seasson);
        false
    } else {
        true
    }
}

fn detect_anomaly(seasson: String, data: &Vec<Register>) {
    for i in 1..data.len() {
        if (data[i].temperature - data[i - 1].temperature).abs() >= 10.0 {
            println!("{} Anomaly Detected:", seasson);
            println!(
                "On day {}, it was {}ºC, but the next day it was {}ºC.",
                data[i - 1].day,
                data[i - 1].temperature,
                data[i].temperature
            );
            println!(
                "which means a temperature change of {}ºC in just one day.",
                data[i - 1].temperature - data[i].temperature
            );
        }
    }
}

fn is_data_empty(data: &Vec<Register>) -> bool {
    if data.len() == 0 {
        println!("No registers to analyze");
        return true;
    } else {
        return false;
    }
}

fn print_menu() {
    println!("1. New register");
    println!("2. See statistics");
    println!("3. Filter by seasson");
    println!("4. Detect anomalies");
    println!("5. Exit");
}

fn process_registers(data: &Vec<Register>) -> (f64, f64, u32) {
    let average = get_average(&data);
    let maximum = get_max_temp(&data);
    let min_day_register = get_minimum_day_temp(&data);
    (average, maximum, min_day_register.day)
}

fn get_minimum_day_temp(data: &Vec<Register>) -> Register {
    let mut minimum_index: usize = 0;
    for i in 1..data.len() {
        if data[i].temperature < data[minimum_index].temperature {
            minimum_index = i;
        }
    }
    let minimum_day = Register {
        day: *(&data[minimum_index].day),
        temperature: *(&data[minimum_index].temperature),
        seasson: data[minimum_index].seasson.clone(),
    };
    minimum_day
}

fn get_max_temp(data: &Vec<Register>) -> f64 {
    let mut maximum: f64 = data[0].temperature;
    for i in 0..data.len() {
        if maximum < data[i].temperature {
            maximum = data[i].temperature;
        }
    }
    maximum
}

fn get_average(data: &Vec<Register>) -> f64 {
    let mut average: f64 = 0.0;
    for i in 0..data.len() {
        average += data[i].temperature;
    }
    average /= data.len() as f64;
    average
}

fn new_register() -> Register {
    let day = request_day();
    let temperature = request_temp();
    let seasson = select_seasson();

    let register = Register {
        day: day,
        temperature: temperature,
        seasson: seasson,
    };

    register
}

fn filter_by_seasson(selected_seasson: String, data: &Vec<Register>) -> Vec<Register> {
    let mut filtered_data: Vec<Register> = Vec::new();

    for i in 0..data.len() {
        if data[i].seasson == *selected_seasson {
            let new_register = Register {
                day: *(&data[i].day),
                temperature: *(&data[i].temperature),
                seasson: selected_seasson.to_string(),
            };
            filtered_data.push(new_register);
        }
    }
    filtered_data
}

fn select_seasson() -> String {
    loop {
        println!("Select a seasson:");
        println!("1. Spring");
        println!("2. Summer");
        println!("3. Autumn");
        println!("4. Winter");
        let mut user_input: String = String::new();

        read_input(&mut user_input);

        let user_input: u32 = user_input.trim().parse().expect("Something went wrong...");

        match user_input {
            1 => {
                println!("Spring selected");
                return String::from("Spring");
            }
            2 => {
                println!("Summer selected");
                return String::from("Summer");
            }
            3 => {
                println!("Autumn selected");
                return String::from("Autumn");
            }
            4 => {
                println!("Winter selected");
                return String::from("Winter");
            }
            _ => {
                println!("Invalid option.");
                continue;
            }
        }
    }
}

fn request_temp() -> f64 {
    #[allow(unused_assignments)]
    let mut temp: f64 = 0.0;

    println!("Introduce the temperature:");
    let mut user_input: String = String::new();
    read_input(&mut user_input);

    temp = user_input.trim().parse().expect("Something went wrong...");
    temp
}

fn request_day() -> u32 {
    #[allow(unused_assignments)]
    let mut day: u32 = 0;
    loop {
        println!("Introduce the day:");
        let mut user_input: String = String::new();
        read_input(&mut user_input);

        day = user_input.trim().parse().expect("Something went wrong...");
        if day > 31 {
            continue;
        }
        break;
    }
    day
}

fn read_input(input: &mut String) {
    io::stdin()
        .read_line(input)
        .expect("Something went wrong...");
}

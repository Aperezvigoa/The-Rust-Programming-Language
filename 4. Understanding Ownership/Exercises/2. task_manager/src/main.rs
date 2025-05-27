use std::io;

fn main() {
    let mut task_list = Vec::new();

    'program: loop {
        menu();

        let mut user_choice: String = String::new();
        own_read_line(&mut user_choice);

        match user_choice.trim() {
            "1" => {
                task_list.push(create_task());
            }
            "2" => {
                if task_list.len() == 0 {
                    println!("No tasks!");
                } else {
                    let selected_task = select_task(&task_list);
                    update_state(&mut task_list[selected_task]);
                }
            }
            "3" => {
                if task_list.len() == 0 {
                    println!("No tasks!");
                } else {
                    let selected_task = select_task(&task_list);
                    update_priority(&mut task_list[selected_task]);
                }
            }
            "4" => {
                if task_list.len() == 0 {
                    println!("No tasks!");
                } else {
                    let selected_task = select_task(&task_list);
                    show_task_details(&task_list[selected_task]);
                }
            }
            "5" => {
                if task_list.len() == 0 {
                    println!("No tasks!");
                } else {
                    let selected_priority = request_priority();
                    filter_by_priority(&task_list, selected_priority);
                }
            }
            "6" => {
                if task_list.len() == 0 {
                    println!("No tasks!");
                } else {
                    most_urgent(&task_list);
                }
            }
            "7" => {
                if task_list.len() == 0 {
                    println!("No tasks");
                } else {
                    calculate_time(&task_list);
                }
            }
            "8" => {
                break 'program;
            }
            _ => {
                println!("Invalid option...");
            }
        }
    }
}

fn menu() {
    println!("=======================");
    println!("1. New Task");
    println!("2. Change State");
    println!("3. Update Priority");
    println!("4. Show Task");
    println!("5. Filter By Priority");
    println!("6. Most Urgent Task");
    println!("7. Calculate Time");
    println!("8. Exit");
    println!("=======================");
    println!("Select a option (1..8):");
}

fn calculate_time(tasks: &[(String, String, u8, u8, u8)]) -> i32 {
    let mut total_time: i32 = 0;

    for t in tasks {
        if t.3 == 2 {
            continue;
        } else {
            total_time += t.4 as i32;
        }
    }
    total_time
}

fn most_urgent(tasks: &[(String, String, u8, u8, u8)]) {
    let mut total_completed = 0;

    for t in tasks {
        if t.3 == 2 {
            total_completed += 1;
        }
    }

    if total_completed == tasks.len() {
        println!("All tasks have been completed.");
    } else {
        let mut temp_incompleted_tasks = Vec::new();
        for t in tasks {
            if t.3 != 2 {
                temp_incompleted_tasks.push(t.clone());
            }
        }
        let mut most_urgent = &temp_incompleted_tasks[0];

        for t in &temp_incompleted_tasks {
            if t.2 > most_urgent.2 {
                most_urgent = t;
            }
        }

        println!("The most urgent task is: {}", most_urgent.1);
    }
}

fn select_task(tasks: &[(String, String, u8, u8, u8)]) -> usize {
    loop {
        print_tasks(tasks);
        println!("Select an option:");

        let mut selected_task: String = String::new();
        own_read_line(&mut selected_task);

        let selected_task: usize = selected_task.trim().parse().expect("Something went wrong!");
        let max: usize = tasks.len();

        match selected_task {
            x if x > 0 && x <= max => {
                return selected_task - 1;
            }
            _ => {
                continue;
            }
        }
    }
}

fn show_task_details(task: &(String, String, u8, u8, u8)) {
    println!("**********************");
    println!("Title: {}", task.1);
    println!("Description: {}", task.0);
    println!(
        "Priority: {} State: {} Hours to complete: {}",
        task.2, task.3, task.4
    );
}

fn print_tasks(tasks: &[(String, String, u8, u8, u8)]) {
    let mut index: u32 = 1;
    for i in tasks {
        println!("{index} - {}", i.1);
        index += 1;
    }
}

fn filter_by_priority(tasks: &[(String, String, u8, u8, u8)], priority: u8) {
    let mut index: u32 = 1;
    for t in tasks {
        if t.2 == priority {
            println!("{} - {}", index, t.1);
            index += 1;
        }
    }
}

fn update_state(task: &mut (String, String, u8, u8, u8)) {
    if task.3 == 2 {
        println!("The task is completed.");
    } else {
        loop {
            let new_state: u8 = request_state();
            if new_state <= task.3 {
                println!("You cannot assign an older or the same state");
                continue;
            } else {
                task.3 = new_state;
                break;
            }
        }
    }
}

fn update_priority(task: &mut (String, String, u8, u8, u8)) {
    if task.3 == 2 {
        println!("You cannot change the priority, the task is completed.");
    } else {
        let new_priority: u8 = request_priority();
        task.2 = new_priority;
        println!("Priority updated.");
    }
}

fn create_task() -> (String, String, u8, u8, u8) {
    let description: String = request_description();
    let title: String = request_title();
    let priority: u8 = request_priority();
    let state: u8 = request_state();
    let time_to_complete: u8 = request_time();

    let task: (String, String, u8, u8, u8) =
        (description, title, priority, state, time_to_complete);
    task
}

fn request_description() -> String {
    println!("Please, enter the task description.");
    let mut description: String = String::new();

    own_read_line(&mut description);
    description
}

fn request_title() -> String {
    println!("Please, enter the task title.");
    let mut title: String = String::new();

    own_read_line(&mut title);
    title
}

fn request_priority() -> u8 {
    loop {
        println!("Assign the priority to the task:");
        println!("1. Very low");
        println!("2. Low");
        println!("3. Medium");
        println!("4. High");
        println!("5. Very high");

        let mut choice: String = String::new();
        own_read_line(&mut choice);

        let choice: u8 = choice.trim().parse().expect("Something went wrong");

        match choice {
            1..=5 => {
                return choice;
            }
            _ => {
                continue;
            }
        }
    }
}

fn request_state() -> u8 {
    loop {
        println!("Set the task state");
        println!("0. Pending");
        println!("1. In Progress");
        println!("2. Completed");

        let mut choice: String = String::new();
        own_read_line(&mut choice);

        let choice: u8 = choice.trim().parse().expect("Something went wrong");

        match choice {
            0..=3 => {
                return choice;
            }
            _ => {
                continue;
            }
        }
    }
}

fn request_time() -> u8 {
    println!("Estimated time to complete task:");
    let mut input: String = String::new();

    own_read_line(&mut input);
    let input: u8 = input.trim().parse().expect("Something went wrong");
    input
}

fn own_read_line(variable: &mut String) {
    io::stdin()
        .read_line(variable)
        .expect("Something went wrong");
}

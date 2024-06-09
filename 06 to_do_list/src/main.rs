use std::io;
use std::io::Write; // To use flush

fn main() {
    let mut tasks: Vec<String> = Vec::new();

    loop {
        println!("What do you want to do?: \n 1. Add \n 2. Remove \n 3. View \n 4. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to get input!");
        let input: i32 = input.trim().parse().expect("Please enter an integer");

        match input {
            1 => add(&mut tasks),
            2 => remove(&mut tasks),
            3 => view(&tasks),
            4 => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn add(tasks: &mut Vec<String>) {
    println!("Enter the task to add:");

    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to get input!");
    let task = task.trim().to_string();

    tasks.push(task);
    println!("Task added successfully.");
}

fn remove(tasks: &mut Vec<String>) {
    view(tasks);
    println!("Enter the number of the task to remove:");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to get input!");
    let index: usize = index.trim().parse().expect("Please enter a valid number");

    if index > 0 && index <= tasks.len() {
        tasks.remove(index - 1);
        println!("Task removed successfully.");
    } else {
        println!("Invalid task number.");
    }
}

fn view(tasks: &Vec<String>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        println!("Your tasks:");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}. {}", i + 1, task);
        }
    }
}

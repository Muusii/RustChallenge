// fn main() {
//     println!("This is day3 workout");
//     println!("My today's goal is to write a todo app and deploy on browser");
//     println!("Let's GO>>>>.");

//     // call functions
//     create();

// }

// use std::io;

// // create todo list function
// fn create() {
//     let mut input = String::new();
//     println!("Create a task todo today");
//     io::stdin().read_line(&mut input) .expect("Read line failed");
    
//     let create = input;
//     println!("Today's List.");
//     println!("{create}");

// }

// //read or view to do list function
// fn view() {
//     let view = create;
//     println!("The current tasks in your list today.");
//     println!("{create}");
// }

use std::io;

fn main() {
    let mut tasks = Vec::new();
    loop {
        println!("Choose an option:");
        println!("1. Create a task");
        println!("2. View tasks");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Invalid input");

        match choice {
            1 => create(&mut tasks),
            2 => view(&tasks),
            3 => break,
            _ => println!("Invalid option. Please try again."),
        }
    }
}

// Create a task function
fn create(tasks: &mut Vec<String>) {
    let mut input = String::new();
    println!("Create a task to do today:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let task = input.trim().to_string();
    tasks.push(task);
    println!("Task added.");
}

// View tasks function
fn view(tasks: &Vec<String>) {
    println!("Today's List:");
    for (index, task) in tasks.iter().enumerate() {
        println!("{}. {}", index + 1, task);
    }
}
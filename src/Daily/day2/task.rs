
fn main() {
    // calling functions here
    create_task();
}

use std::io;
// create task function
fn create_task() {
    let mut today_task_input = String::new();
    println!("My Todays tasks are the following: ");
    io::stdin().read_line(&mut today_task_input) .expect("Read line failed");

    // the immutable task
    let today_task = today_task_input;
    println!("ToDo list for today is: {today_task}");

}
fn main() {
    println!("Good Morning");
    // cll function
    prompt();
    prompt_immutable();
}

use std::io;
fn prompt() {
    let mut need = String::new();
    println!("What's your need");
    // let mut need2 = String::new();
    io::stdin().read_line( &mut need) .expect("Read line failed");
    println!("Your need is {need}");
}

fn prompt_immutable() {
    let mut input = String::new();
    println!("The second need is?");
    io::stdin(). read_line(&mut input) .expect("Read line failed");
    
    //the immutable var now
    let need2 = input;
    println!("Your second need is: {need2}");
}
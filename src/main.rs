fn main() {
    println!("Hello, world!");

    // calling another function
    joy();
    lib();
    passing_value(20,"real functions");
    loops();
    vars();
    calculation();
    joy();
}
fn joy() {
    println!("Hello Girl, day one here let's us rock this challenge!!!!!!!");
}
fn calculation() {
     //simple maths
     let add = 3+5;
     let _substract = 5-3;
     let _multiply = 5*3;
     let _modulus = 20%3;
     let _divide = 20/5;
     println!("add is: {add}");
}
fn vars() {
    // variables

    let x = 6;
    println!("The value of x is: {x}");
    let _x = 6;
    let _x = x+2;
    println!("The value of x is: {x}");
}
fn lib() {
    println!("this is a second func");
}
fn passing_value(y: i32, test: &str) {
    // let y = 20;
    // let test = "real functions"
    println!("The testing is: {y} {test}")
}
fn loops() {
    // example to test my understanding
    let weather = "rainy";
    while weather=="rainy"{
        println!("Stay indoors");
        break;
    }
    
}
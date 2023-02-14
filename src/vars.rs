// Variables hold primitive data or references
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "env";
    let age = 25;

    let mut date = 31012023;

    // age = 24; // cannot do this (immutable)

    println!("My name is {}, I'm {}, made for {}", name, age, date);

    date = 31012024; // can do!
    println!("test on {}", date);

    // define constants
    const ID: i32 = 001; // type should be mentioned
    println!("ID: {}", ID);

    // assign multiple vars
    let (my_name, my_age) = ("fauv", 24);
    println!("{}, {}", my_name, my_age);
}

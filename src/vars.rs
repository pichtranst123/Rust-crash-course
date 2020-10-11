// Variables hold primitive data or references to data
// Variables are immutable be default
// Rust is a block-scoped language

pub fn run() {
    let name = "Peter";
    let mut age = 24;
    println!("My name is {} and I am {}", name, age);

    age = 23;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Define multiple  vars
    let (my_name, my_age) = ("Peter", 24);
    println!("{} is {}", my_name, my_age);
}

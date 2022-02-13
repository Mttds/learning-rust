// Variables are immutable by default (like in functional programming)
// Rust is a block-scoped language

pub fn run() {
    let name = "Brad";
    let age = 30;
    let mut age2 = 30;

    //age = 31; // cannot do this! immutable
    age2 = 31; // this is allowed because we use mut
    println!("My name is {} and I am {}", name, age);
    println!("My name is {} and I am {}", name, age2);

    // Define constants
    // we need to add a type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 31);
    println!("{} is {}", my_name, my_age);
}
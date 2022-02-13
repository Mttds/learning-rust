pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    // Basic formatting
    println!("Number: {}", 1); // like .format in python
    println!("{} is from {}", "Dave", "USA");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to eat", "Dave", "USA");

    // Name arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Basketball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    // Prints the tuple/array
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
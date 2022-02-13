// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify
// or own string data

pub fn run() {
    let hello = "Hello"; // primitive str
    let mut hello2 = String::from("Hello"); // Growable heap-allocated data structure
    println!("{}", hello2);

    // Get length
    println!("Len: {}", hello.len());
    println!("Len: {}", hello2.len());

    hello2.push('W'); // append a character
    hello2.push_str("orld"); // append a string
    println!("{}", hello2);

    // Capacity in bytes for a String::
    println!("Capacity: {}", hello2.capacity());

    // Other methods
    println!("Is Empty?: {}", hello2.is_empty());
    println!("Contains World?: {}", hello2.contains("World"));
    println!("Replace: {}", hello2.replace("World", " There"));

    // Loop thru string by whitespace
    for w in hello2.replace("World", " There").split_whitespace() {
        println!("{}", w);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing. Shows an error if it fails
    assert_eq!(3, s.len());
    println!("{}", s);
}
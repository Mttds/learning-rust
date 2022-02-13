/*
Primitive types:
Integers: u8, i8, ..., u128, i128
Floats: f32, f64
Boolean: bool
Characters: char
Tuples
Arrays (Fixed length)
*/

// Rust is a statically typed language, which means that
// it must know the types of all variables at compile time.
// However the compiler can infer what type we want to use based
// on the value and how we use it (i.e let var = 1 makes it an i32)

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let f = 2.5;

    // Add explicit type
    let y: i64 = 142041204;

    // Max size of a datatype
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;
    let expression = x > y;
    println!("{:?}", (x, f, y, is_active, expression));

    // Char
    let a1 = 'a'; // cannot be more than a character (i.e ab). can be any unicode character
    let face = '\u{1F600}';
    println!("{}",face);
}
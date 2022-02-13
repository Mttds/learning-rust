// Vectores - Resizeable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", numbers);

    // Re-assign a value
    numbers[2] = 20;

    // Add on to a vector
    numbers.push(6);
    println!("{:?}", numbers);

    numbers.pop();
    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[0]);

    // Get vector len
    println!("Vectore len: {}", numbers.len());

    // Vectors are stack allocated
    // can also be std::mem::size_of_val if we don't use std::mem;
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slices
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Vector iteration
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Vector iteration & mutation
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
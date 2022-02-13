// Arrays - Fixed list where elements have the same data type

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);

    // Re-assign a value
    numbers[2] = 20;

    // Get single value
    println!("{}", numbers[0]);

    // Get array len
    println!("Array len: {}", numbers.len());

    // Arrays are stack allocated
    // can also be std::mem::size_of_val if we don't use std::mem;
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slices
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
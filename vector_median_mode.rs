use std::io;
use std::collections::HashMap;

fn main() {
    println!("=== Vector Median & Mode ===");
    println!("Please input a list of numbers. Press Enter once done.");

    let mut input_vec: Vec<i32> = Vec::new();

    loop {
        println!("Please enter an integer or press Enter if you are done.");
        let mut _input: String = String::new();
        io::stdin().read_line(&mut _input).expect("Error while reading input");

        if _input == "\n" {
            break;
        }

        let _input = match _input.trim().parse() {
            Ok(i) => input_vec.push(i),
            Err(_) => { println!("Wrong input, please try again or press Enter if you are done!"); continue }
        };
    }

    println!("Your vector: {:?}", input_vec);
    let vec_len: usize = input_vec.len();
    let mut vec_element_count = HashMap::new();
    let mut median: Option<f32> = None;

    if vec_len % 2 == 0 {
        median = Some((input_vec[vec_len/2 - 1] + input_vec[vec_len/2]) as f32 / 2.0);
    }
    else {
        median = Some(input_vec[vec_len/2] as f32); // integer division here, if vec_len = 5 then we get 2 which is the 3rd element (0 1 [2] 3 4) which is the median
    }
    
    match median {
        Some(num) => println!("The median is: {}", num),
        None => println!("Could not calculate the median.")
    }

    for i in input_vec {
        /* 
        For every element in the vector insert it into the map with the value as the key
        and 1 as the value. If already present just add 1 to the value of the corresponding key.
        We have to dereference count because or_insert returns a reference to the value of the key in the map.
        */
        let count = vec_element_count.entry(i).or_insert(0);
        *count += 1;
    }

    /* 
    Actually we should handle the case where multiple keys have the same value
    because if there is no single element that appears more often in the vector
    then this expression will return the last encountered in the hashmap which is
    unordered. For example with [1,2,3] it can return either 1 or 2 or 3. In this
    case it would be better to return None because there is no mode.
    */
    let mode = vec_element_count.iter().max_by_key(|entry | entry.1).unwrap();
    println!("The mode is: {:?}", mode.0);
}
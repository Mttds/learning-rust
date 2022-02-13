pub fn run() {
    greeting("Hello", "Jane");

    // Bind function values to variables
    let get_sum = add(5,5);
    println!("Sum: {}", get_sum);

    // Closure
    // Can also use outside variables that we
    // cannot use in functions because they are block-scoped
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("Closure sum: {}", add_nums(3,3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 { // returs and i32
    n1 + n2 // if we don't use the semi-colon this is like return
}
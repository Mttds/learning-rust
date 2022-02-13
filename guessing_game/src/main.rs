use std::io;
use rand::Rng; // from the downloaded crate rand
use std::cmp::Ordering; // Ordering enum

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // convert to u32 by shadowing the previous guess variable with a new one
        // parse returns a Result type which can return Ok and Err as its variants (enum)
        // we can match the enum returned with match and handle the Ok/Err enum variants
        // if Ok then number (num) will be returned
        // if Err we continue
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

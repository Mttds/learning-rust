pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // If/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    }
    else if age < 21 && check_id {
        println!("You are not old enough to drink!");
    }
    else {
        println!("I'll need to see your ID");
    }

    // Shorthand if, similar to ternary operator
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}", is_of_age);
}
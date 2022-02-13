// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct ColorT(u8, u8, u8);

// Struct with methods (i.e simila to a class)
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct a Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn get_fullname(&self) -> String {
        // format! is a macro similar to println! but it does not print
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color{red: 255, green: 0, blue: 0};
    println!("Color: {} {} {}", c.red, c.green, c.blue);
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut ct = ColorT(255, 0, 0);
    println!("Color: {} {} {}", ct.0, ct.1, ct.2);
    ct.0 = 195;
    println!("Color: {} {} {}", ct.0, ct.1, ct.2);

    let mut p = Person::new("John", "Doe");
    let mut p2 = Person::new("Jane", "Doe");
    p2.set_last_name("Williams");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("The full name is: {}", p.get_fullname());
    println!("The full name is: {}", p2.get_fullname());
    println!("Person tuple: {:?}", p2.to_tuple());
}
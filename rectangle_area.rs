// implements the Debug trait for our struct
// this allows us to use println! macro with {:?} or {:#?} for indent
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// we could also have multiple impl blocks for the same structure
// but here it doesn't make sense
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
        || 
        self.width >= rect.height && self.height >= rect.width
    }

    // Does not have self as a parameter because
    // it creates a new Rectangle object, but
    // it does not use data from a one
    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 5,
        height: 10
    };

    let rect1 = Rectangle {
        width: 6,
        height: 5
    };

    let square1 = Rectangle::square(5);

    println!(
        "The area of the rectangle {:?} is: {}",
        rect,
        rect.area()
    );

    println!(
        "The area of the square {:?} is: {}",
        square1,
        square1.area()
    );

    println!(
        "Rectangle {:?} can hold rectangle {:?}: {}",
        rect,
        rect1,
        rect.can_hold(&rect1)
    );
}

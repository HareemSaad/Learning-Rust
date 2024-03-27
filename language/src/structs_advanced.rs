#[derive(Debug)] // trait so this struct can be printed using println!
struct Rectangle {
    width: u32,
    height: u32,
}

// associated functions
impl Rectangle { // impl struct_name: implementation block (contains all methods)
    // Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably (&mut self), just as they can any other parameter.
    fn area(&self) -> u32 { // method function &self (borrows the Self (calling) instance)
        self.width * self.height
    }

    // other: &Rectangle: takes a reference to an existing rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn structs_advanced() {
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}\n", rect1); // prints struct in single line
    println!("rect1 is {:#?}\n", rect1); // prints struct in multiple lines with formatiing

    dbg!(&rect1); // prints it with file and line number

    println!(
        "The area of the rectangle is {} square pixels.\n",
        rect1.area()
    );
    
    let scale = 9;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
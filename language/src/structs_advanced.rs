#[derive(Debug)] // trair so this struct can be printed using println!
struct Rectangle {
    width: u32,
    height: u32,
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
        area(&rect1)
    );

    let scale = 9;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
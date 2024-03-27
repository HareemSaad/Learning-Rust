#[derive(Debug)]
enum Shape {
    CIRCLE,
    SQUARE,
    OVAL
}

pub fn enums() {
    
    let circle = Shape::CIRCLE;
    println!("Circle is {:?}", circle);
}
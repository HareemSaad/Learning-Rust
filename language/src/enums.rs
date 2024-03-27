#[derive(Debug)]
enum Shape {
    CIRCLE,
    SQUARE,
    OVAL
}

#[derive(Debug)]
enum Color {
    GOLD,
    SILVER,
    BRONZE
}

#[derive(Debug)]
struct Medal {
    shape: Shape,
    color: Color
}

const WINNER_MEDAL: Medal = Medal {
    shape: Shape::CIRCLE,
    color: Color::GOLD,
};

const RUNNER_UP_MEDAL: Medal = Medal {
    shape: Shape::SQUARE,
    color: Color::SILVER,
};

const SECOND_RUNNER_UP_MEDAL: Medal = Medal {
    shape: Shape::OVAL,
    color: Color::BRONZE,
};

#[derive(Debug)]
enum Prize {
    WINNER(Medal), // only type defination allowed in ()
    RUNNER_UP(Medal),
    SECOND_RUNNER_UP(Medal)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // named fields in {}
    Write(String), // type string 
    ChangeColor(i32, i32, i32), // tuple
}

pub fn enums() {
    
    let circle = Shape::CIRCLE;
    println!("Circle is {:?}", circle);

    let winner_prize = Prize::WINNER(WINNER_MEDAL);
    let runner_up_prize = Prize::RUNNER_UP(RUNNER_UP_MEDAL);
    let second_runner_up_prize = Prize::SECOND_RUNNER_UP(SECOND_RUNNER_UP_MEDAL);

    println!("{:?}", winner_prize);
    println!("{:?}", runner_up_prize);
    println!("{:?}", second_runner_up_prize);
}
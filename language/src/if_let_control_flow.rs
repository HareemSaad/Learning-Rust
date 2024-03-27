enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

pub fn if_let_control_flow() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // transform into an `if let` statement
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } 

    // you cannot use the same coin variable for the 2 below code blocks, because match will take ownership of coin,
    // you can either re initialize the val or use reference '&' or implement the copy trait by writing #[derive(Debug, Clone, Copy)] above both enums
    let coin: Coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match &coin { // use reference
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    
    // or
    
    // let coin: Coin = Coin::Quarter(UsState::Alabama); // reinitialize
    if let Coin::Quarter(state) = &coin { // use refernce
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}
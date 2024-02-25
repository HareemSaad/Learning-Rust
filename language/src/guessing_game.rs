use std::io; // standard input output library
use rand::Rng; // Rng is a trait

pub fn game() {
    println!("Guess the number!");

    // thread_rng is the generator we are going to use: one that is local to the current thread of execution and is seeded by the operating system.
    // The gen_range method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // create a new string

    // use :: to call associated function
    io::stdin() // invoke standard input
        .read_line(&mut guess)  // read line of text from user and store it by passing address of the variable
        // writing &mut allows us to modify `guess` variable (allows to use mutability)
        .expect("Failed to read line"); 

    println!("You guessed: {guess}");
}
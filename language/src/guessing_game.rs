use std::io; // standard input output library
use std::cmp::Ordering;
use rand::Rng; // Rng is a trait

pub fn game() {
    println!("Guess the number!");

    // thread_rng is the generator we are going to use: one that is local to the current thread of execution and is seeded by the operating system.
    // The gen_range method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form start..=end and is inclusive on the lower and upper bounds, so we need to specify 1..=100 to request a number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    // loop
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // create a new string

        // use :: to call associated function
        io::stdin() // invoke standard input
            .read_line(&mut guess)  // read line of text from user and store it by passing address of the variable
            // writing &mut allows us to modify `guess` variable (allows to use mutability)
            .expect("Failed to read line"); 

        // The trim method on a String instance will eliminate any whitespace at the beginning and end
        // The parse method on strings converts a string to another type. We need to tell Rust the exact number type we want by using let guess: u32
        //  If parse returns an Err Result variant because it couldn’t create a number from the string, the expect call will crash the game and print the message we give it. If parse can successfully convert the string to a number, it will return the Ok variant of Result, and expect will return the number that we want from the Ok value.
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // loop break
            },
        }
    }
}
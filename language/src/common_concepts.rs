use std::io; // standard input output library

pub fn common_concepts() {
    
    println!("\nMutability: If you want to re write a variable use the mut keyword");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    println!("\nConstants: Like immutable variables, constants are values that are bound to a name and are not allowed to change");
    // differneces between immutable variables and contstants 
    // First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.
    // Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
    // The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    if THREE_HOURS_IN_SECONDS != 0 {
        // accessable in child scopes
        println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");
    }

    println!("\nShadowing: rewriting mutable variables");
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6
    
    // we cannot do this if spaces is mutable because then it would expect same type value
    let spaces = "   "; // type = string
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len(); // type = number
    println!("The value of spaces is: {spaces}");
}
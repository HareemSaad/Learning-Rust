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

    // Integer types
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize // 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

    // In Rust, attempting to assign a value outside the valid range of a u8 (0-255) leads to integer overflow. In debug mode, this causes a program to panic and exit. In release mode, Rust employs two’s complement wrapping, where values beyond the maximum loop back to the minimum (e.g., 256 becomes 0). This behavior might not match your expectations and is generally considered incorrect. To manage overflow, Rust offers methods like wrapping_* for wrapping, checked_* for returning None on overflow, overflowing_* for indicating overflow with a boolean, and saturating_* for capping values at the type’s limits.

    // floating types - f32 and f64
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Operations
    println!("\nOperations");

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("The value of truncated is: {}", truncated);
    let truncated = -5.0 / 3.0; // Results in -1.66
    println!("The value of truncated is: {}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {}", f);

    // use '' for chars "" for strings
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!(
        "The values of c, z, and heart_eyed_cat are: {}, {}, {}",
        c, z, heart_eyed_cat
    );

    // The compund types
    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The tuple is {:?}", tup);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    println!("The value of five_hundred is: {}", five_hundred);

    let six_point_four = tup.1;
    println!("The value of six_point_four is: {}", six_point_four);

    let one = tup.2;
    println!("The value of one is: {}", one);

    // arrays
    let a = [1, 2, 3, 4, 5];
    println!("The array a is {:?}" ,a);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The array a is {:?}" ,a);
    let a = [3; 5];
    println!("The array a is {:?}" ,a);
    println!("Accessing first element: {}", a[0]);

    println!("\nFunctions");
    print_labeled_measurement(5, 'h');
    let x = plus_one(5);

    println!("The value of x is: {x}");

    println!("\nStatements and Expressions");
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value and return a value.

    let y = {
        let x = 3; // statement
        x + 1 // expression - no semicolon add one and its a statement
    };

    println!("The value of y is: {y}");

    println!( "\nControl Flow" );
    let number = 6;

    if number % 4 == 0 { // if expects bool if a var named val = 4 if val {} will result in an error
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
fn plus_one(x: i32) -> i32 {
    x + 1 // no semicolon on a expression (expression is a return statement)
}
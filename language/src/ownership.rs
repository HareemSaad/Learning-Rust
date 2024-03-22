use std::io; // standard input output library

pub fn ownership() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        println!("{}", s);  // this will print `hello` because `s` is in scope
    }                      // this scope is now over, and s is no longer valid
    // println!("{}", s);     // this will NOT work

    let mut s = String::from("hello");
    // let s = "hello"; // this is a literal; literals cannot be mutated, strings can be

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient.
    // In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient.
    //  - The memory must be requested from the memory allocator at runtime. (done when calling `String::from`)
    //  - We need a way of returning this memory to the allocator when we’re done with our String. (done when variable goes out of scope)

    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2); // prints `hello, world!`,
    // println!("{}", s1); // value is moved from s1 to s2 s1 does not exist

    // A String is made up of three parts, a pointer to the heap memory where the string lives, a length, and capacity. This group of data is stored on the stack.
    // The length is how much memory, in bytes, the contents of the String are currently using. The capacity is the total amount of memory, in bytes, that the String has received from the allocator.
    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to. 
    // This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.

    // If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone. expensive.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
    // sine integers are stored on stack, there’s no reason we would want to prevent x from being valid after we create the variable y.

    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

    // note that we pass &s1 into calculate_length and, in its definition, we take &String rather than String. These ampersands represent references, and they allow you to refer to some value without taking ownership of it. Fance words for its a pointer
    // Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.
    let mut s1 = String::from("hello"); //here (1)

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s); //here (2)
    
    println!("{}", s1);

    // borrwing twice will cause error, to prevent data race
    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // how to borrow twice then? solution scopes

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // mutable and immutable refernces
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    // println!("{}", r1); // but if this eists meaning r1's scope hasn't eneded and will cause error. reference’s scope starts from where it is introduced and continues through the last time that reference is used. 


}

// reading references
fn calculate_length(s: &String) -> usize {
    s.len()
}

// writing references
// won't work if if the passed string is not mutable; solution add mut
fn change(some_string: &mut String) { // and here (3)
    some_string.push_str(", world");
}
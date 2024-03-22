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


}
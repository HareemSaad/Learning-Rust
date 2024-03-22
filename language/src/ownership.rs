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
    
}
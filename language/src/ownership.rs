use std::io; // standard input output library

pub fn ownership() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        println!("{}", s);  // this will print `hello` because `s` is in scope
    }                      // this scope is now over, and s is no longer valid
    // println!("{}", s);     // this will NOT work

}
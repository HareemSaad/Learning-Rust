struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn structs() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{}",  user1.email);
    
    user1.email = String::from("anotheremail@example.com");
    println!("{}",  user1.email);
}

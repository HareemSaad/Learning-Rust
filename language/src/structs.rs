struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn structs() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );
    println!("{}",  user1.email);
    
    user1.email = String::from("anotheremail@example.com");
    println!("{}",  user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // since parameter names = struct field names we can just write `username.`
        email: email, // since parameter names = struct field names we can just write `email.`
        sign_in_count: 1,
    }
}

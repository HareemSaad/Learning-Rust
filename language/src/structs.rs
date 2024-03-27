struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn structs() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );
    println!("{}",  user1.email);
    
    user1.email = String::from("anotheremail@example.com");
    println!("{}",  user1.email);

    // creates an instance in user2 that has a different value for email but has the same values for the username, active, and sign_in_count fields from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}",  user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username, // since parameter names = struct field names we can just write `username.`
        email: email, // since parameter names = struct field names we can just write `email.`
        sign_in_count: 1,
    }
}

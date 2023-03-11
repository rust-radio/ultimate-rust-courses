use std::fmt;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "User {{ 
                active: {}, 
                username: {}, 
                email: {}, 
                sign_in_count: {} 
            }}",
            self.active, self.username, self.email, self.sign_in_count )
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("1 user1: {}", user1);
    user1.email = String::from("anotheremail@example.com");
    println!("2 user1: {}", user1);

    let user2 = build_user(String::from("anotheruser@example.com"), String::from("anotherusername123"));
    println!("user2 is: {}", user2);
    println!("user2.email is: {}", user2.email);
}

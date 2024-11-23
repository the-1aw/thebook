struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u8, u8, u8);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someemail@email.com"),
        sign_in_count: 1,
    };

    let user = build_user(String::from(user.email), String::from(user.username));

    let user2 = User {
        username: String::from("anotherusername"),
        ..user
    };
    println!("user active: {}", user2.active);
    println!("user username: {}", user2.username);
    println!("user sign_in_count: {}", user2.sign_in_count);
    let black = Color(0, 0, 0);
    println!("black is: #{},{},{}", black.0, black.1, black.2);
}

fn main() {
    struct User {
        useranme: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    };

    let user1 = build_user(String::from("user@host.com"), String::from("user"));

    println!("{}", user1.email);

    let user2 = User {
        email: String::from("user2@host.com"),
        username: String::from("user2"),
        ..user1
    }
}

fn build_user(email:String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

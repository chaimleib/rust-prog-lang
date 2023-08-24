struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn display_user(u: &User) {
    println!("{}: {}, active:{}, sign_in_count:{}",
             u.username, u.email, u.active, u.sign_in_count);
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someuser"));

    let user2 = User {
        email: String::from("anotheruser@example.com"),
        ..user1
    };

    // display_user(&user1); // invalid, partial move
    display_user(&user2);
}

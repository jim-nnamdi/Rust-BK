struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        name: String::from("samuel jim"),
        email: String::from("jimsamuel50@gmail.com"),
        active: true,
        sign_in_count: 0,
    };

    build_user(user1.email, user1.name, user1.active, user1.sign_in_count);
}

fn build_user(email: String, name: String, is_active: bool, log_in_count: u64) -> User {
    User {
        email: email,
        name: name,
        active: is_active,
        sign_in_count: log_in_count,
    }
}

fn main() {
    let user1 = build_user(String::from("Alice"), String::from("Alice@bob.com"));

    let user2 = User {
        email: String::from("Charlie@lachocolaterie.com"),
        username: String::from("Charlie"),
        ..user1 // On copie le reste depuis user1
    };

    let black = Color(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i8, i8, i8);

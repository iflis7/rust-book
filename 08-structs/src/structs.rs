#[allow(unused)]

// Struct Definition:
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn structing() {
    println!("**** Structs ****");

    // Creating Instances:
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("user1@email.com"),
        sign_in_count: 1,
    };

    // Accessing and Modifying Fields:
    println!("This is the email of user1: {}", user1.email); // Accessing email
    let mut user1 = user1;
    user1.email = String::from("newemail@example.com"); // Changing email

    // Struct Update Syntax:
    let user2 = User {
        email: String::from("user2@email.com"),
        ..user1
    };

    println!("This is the email of user2: {}", user2.email);

    let user3 = build_user(String::from("user3@email.com"), String::from("user-3"));
    println!("\nThis is the email of user3: {}", user3.email);
}

// Field Init Shorthand:
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

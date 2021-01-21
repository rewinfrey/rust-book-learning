#[derive(Debug)]
struct User {
    username: String, // Using String means the User instance "owns" its username and email fields. We could use &str (string slice) to share this, instead. This would require the use of lifetimes in Rust to share that data in the User struct.
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // when field names are the same as parameter names, we can reuse the parameter without duplicating.
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Structs can also be defined as tuple structs:
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("someone2@example.com"), String::from("someusername1234"));

    // We can also create users from other users' field values.
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // Including a shorthand syntax for taking the remaining fields from a user struct:
    let user4 = User {
        email: String::from("another4@example.com"),
        username: String::from("anotherusername568"),
        ..user1
    };

    println!("user1: {:?}", user1);
    println!("user2: {:?}", user2);
    println!("user3: {:?}", user3);
    println!("user4: {:?}", user4);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!("black: {:?}", black);
    println!("origin: {:?}", origin);
}

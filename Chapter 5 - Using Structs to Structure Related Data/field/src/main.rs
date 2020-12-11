#[derive(Debug)]
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

fn main() {
    println!("Hello, world!");

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        ..user1
    };

    let user3 = build_user(
        String::from("third@example.com"),
        String::from("thirdweeler"),
    );

    user2.email = String::from("anotheremail@example.com");
    println!(
        "User1: {:#?}\nUser2: {:#?}\nUser3: {:#?}",
        user1, user2, user3
    );
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}\n{:?}", black, origin);
}

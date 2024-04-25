// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//     println!("User 1 was created.");
//     println!(
//         "{}, {}, {}, {}",
//         user1.active, user1.username, user1.email, user1.sign_in_count
//     );
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//     println!("User 1 was created.");
//     user1.email = String::from("anotheremail@example.com");
//     println!("User 1, email field was changed.");
//     println!(
//         "{}, {}, {}, {}",
//         user1.active, user1.username, user1.email, user1.sign_in_count
//     );
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = build_user(String::from("test@test.com"), String::from("test123"));
//     println!("User 1 was created.");
//     println!(
//         "{}, {}, {}, {}",
//         user1.active, user1.username, user1.email, user1.sign_in_count
//     );
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = build_user(String::from("test@test.com"), String::from("test123"));
//     println!("User 1 was created.");
//     println!(
//         "{}, {}, {}, {}",
//         user1.active, user1.username, user1.email, user1.sign_in_count
//     );
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let user1 = build_user(String::from("test@test.com"), String::from("test123"));
//     println!("User 1 was created.");
//     let user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
//     println!(
//         "{}, {}, {}, {}",
//         user1.active, user2.username, user1.email, user1.sign_in_count
//     ); // user1.username is borrowed by user2
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     println!("black: {} {} {}", black.0, black.1, black.2);
//     println!("origin: {} {} {}", origin.0, origin.1, origin.2);
// }

struct AlwaysEqual;

fn main() {
    let _subject = AlwaysEqual;
    println!("subject created");
}

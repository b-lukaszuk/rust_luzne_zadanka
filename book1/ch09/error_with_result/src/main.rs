// use std::fs::File;
// fn main() {
//     let greeting_file_result = File::open("hello.txt");
//     println!("{}", greeting_file_result);
// }

// use std::env::current_dir;
// use std::fs::File;
// fn main() {
//     println!("{:?}", current_dir());
//     let greeting_file_result = File::open("./hello1.txt");
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => panic!("Problem opening the file: {:?}", error),
//     };
//     println!("{:?}", greeting_file);
// }

// use std::fs::File;
// use std::io::ErrorKind;
// fn main() {
//     let greeting_file_result = File::open("./hello.txt");
//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello1.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating teh file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };
//     println!("{:?}", greeting_file);
// }

// use std::fs::File;
// fn main() {
//     let greeting_file = File::open("hello1.txt").unwrap();
//     println!("{:?}", greeting_file);
// }

// use std::fs::File;
// fn main() {
//     let greeting_file =
//         File::open("hello1.txt").expect("hello.txt should be included in this project");
//     println!("{:?}", greeting_file);
// }

// use std::fs::File;
// use std::io::{self, Read};
// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
//     let username_file_result = File::open(file_name);
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }
// fn main() {
//     let user_name = read_username_from_file("hello.txt");
//     println!("{:?}", user_name);
//     let user_name = read_username_from_file("hello1.txt");
//     println!("{:?}", user_name);
// }

// use std::fs::File;
// use std::io::{self, Read};
// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
//     let mut username_file = File::open(file_name)?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }
// fn main() {
//     let user_name = read_username_from_file("hello.txt");
//     println!("{:?}", user_name);
//     let user_name = read_username_from_file("hello1.txt");
//     println!("{:?}", user_name);
// }

// use std::fs::File;
// use std::io::{self, Read};
// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open(file_name)?.read_to_string(&mut username)?;
//     Ok(username)
// }
// fn main() {
//     let user_name = read_username_from_file("hello.txt");
//     println!("{:?}", user_name);
//     let user_name = read_username_from_file("hello1.txt");
//     println!("{:?}", user_name);
// }

// use std::fs;
// use std::io;
// fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
//     fs::read_to_string(file_name)
// }
// fn main() {
//     let user_name = read_username_from_file("hello.txt");
//     println!("{:?}", user_name);
//     let user_name = read_username_from_file("hello1.txt");
//     println!("{:?}", user_name);
// }

// use std::fs::File;
// fn main() {
//     // error main returns (), not a Result
//     let greeting_file = File::open("hello.txt")?;
//     println!("{:?}", greeting_file);
// }

// fn last_char_of_first_line(text: &str) -> Option<char> {
//     text.lines().next()?.chars().last()
// }
// fn main() {
//     // error main returns (), not a Result
//     let last_char = last_char_of_first_line("hello");
//     println!("{:?}", last_char);
// }

use std::error::Error;
use std::fs::File;
fn main() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;
    Ok(())
}

// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main() {
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
//     println!("Home address {:?} {}", home.kind, home.address);
//     println!("Loopback address {:?} {}", loopback.kind, loopback.address);
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));
//     println!("Home address {:?}", home);
//     println!("Loopback address {:?}", loopback);
// }

// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);
//     let sum = x + y; // error no implementation for i8 + Option<i8>
//     println!("the result", sum);
// }

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("some_char: {:?}", some_char);
    println!("absent_number: {:?}", absent_number);
}

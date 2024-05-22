// types of references
// &i32 // a reference
// &'a i32 // a reference with na explicit lifetime
// &'a mut i32 // a mutable reference with na explicit lifetime

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// fn main() {
//     let string1 = String::from("long string is long");
//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is '{}'", result);
//     }
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is '{}'", result);
// }

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str() // error, returns a reference, result is dropped when function terminates
}
fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is '{}'", result);
}

// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6; // error, cannot change immutable variable
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("Number of seconds in three hours: {THREE_HOURS_IN_SECONDS}")
// }

// fn main() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let mut spaces = "       ";
//     spaces = spaces.len(); // error, changed type of spaces
//     println!("The length of spaces is: {spaces}")
// }

fn main() {
    let spaces = "       ";
    let spaces = spaces.len(); // no error, changed type but shadowing
    println!("The length of spaces is: {spaces}")
}

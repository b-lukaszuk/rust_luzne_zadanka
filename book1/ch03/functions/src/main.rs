// fn another_function() {
//     println!("Another function");
// }

// fn main() {
//     println!("Hello, world!");
//     another_function();
// }

// fn another_function(x: i32) {
//     println!("In another function",);
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     another_function(5);
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {}{}", value, unit_label);
// }

// fn main() {
//     print_labeled_measurement(5, 'h')
// }

// fn main() {
//     let y = 6; // statement
//     println!("{}", y);
// }

// fn main() {
//     let x = (let y = 6); // error, cannot assign value of a statement
//     println!("{}", y);
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1 // expression
//     };
//     println!("The value of y is: {}", y);
// }

// fn five() -> i32 {
//     5 // expression, returns 5
// }

// fn main() {
//     let x = five();
//     println!("The value of x is: {}", x);
// }

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// fn plus_one(x: i32) -> i32 {
//     x + 1; // this is a statement, nothing to be returned
// }

// fn main() {
//     let x = plus_one(5);
//     println!("The value of x is: {}", x);
// }

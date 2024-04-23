// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // error immutable ref in args
// }

// fn main() {
//     let s = String::from("hello");
//     change(&s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
// }

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s; // error, can have only 1 active mutable referece
//     println!("{}, {}", r1, r2);
// }

// fn main() {
//     let mut s = String::from("hello");
//     {
//         let r1 = &mut s;
//         println!("{}", r1);
//     } // r1 goes out of scope here
//     let r2 = &mut s;
//     println!("{}", r2);
// }

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM
//     println!("{}, {}, and {}", r1, r2, r3);
// }

// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     let r3 = &mut s; // no problem
//     println!("{}", r3);
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // error, s goes out of scope
// }

// fn main() {
//     let reference_to_nothing = dangle();
//     println!("{}", reference_to_nothing);
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    let reference_to_something = no_dangle();
    println!("{}", reference_to_something);
}

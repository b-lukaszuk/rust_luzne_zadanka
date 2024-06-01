// use std::{thread, time::Duration};
// fn main() {
//     let expensive_closure = |num: u32| -> u32 {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
// }

// fn main() {
//     fn add_one_v1(x: u32) -> u32 {
//         x + 1
//     }
//     let add_one_v2 = |x: u32| -> u32 { x + 1 };
//     let add_one_v3 = |x| x + 1; // or {x + 1}
// }

fn main() {
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5); // error, based on line above x is inferred to be String
}

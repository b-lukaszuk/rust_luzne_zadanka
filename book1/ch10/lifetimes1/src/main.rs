// dangling references
// fn main() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r: {}", r);
// }

// // borrow checker 1
// fn main() {
//     let r; // -- begin 'a
//     {
//         let x = 5; // -- begin 'b
//         r = &x;
//     } // end 'b
//     println!("r: {}", r);
// } // end 'a

// // borrow checker 2
// fn main() {
//     let x = 5; // -- begin 'a
//     let r = &x; // -- begin 'b
//     println!("r: {}", r);
// } // end 'a 'b

// generic lifetimes in functions
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} // error, rust does not know whether reference returned belongs to x or to y

fn main() {
    println!("Hello world!");
}

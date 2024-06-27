// fn main() {
//     let b = Box::new(5);
//     println!("b = {}", b);
// }

// non-recursive type
// Rust must reserve space for the greatest size elt of Message
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// recursive type
// Rust does not know how much space to reserve
// List inside is theoretically infinitive
// enum List {
//     Cons(i32, List),
//     Nil,
// }
// use crate::List::{Cons, Nil};
// fn main() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
// }

// recursive type
// now, list from Box is on heap
// Rust must only reserve space on heap for pointers (Box) to List (on healp)
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("The recursive list compiles");
}

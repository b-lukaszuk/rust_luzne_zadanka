// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
// use crate::List::{Cons, Nil};
// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a)); // moves ownership of a to b
//     let c = Cons(4, Box::new(a)); // cannot use a, b is the owner of the list
// }

use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use crate::List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

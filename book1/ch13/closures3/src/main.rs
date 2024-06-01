// closures can capture values in 3 ways:
// borrowing immutably,
// borrowing mutably,
// taking ownership.

// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);
//     let only_borrows = || println!("From closure: {:?}", list);
//     println!("Before calling closure: {:?}", list);
//     only_borrows();
//     println!("After calling closure: {:?}", list);
// }

// fn main() {
//     let mut list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);
//     let mut borrows_mutably = || list.push(7);
//     borrows_mutably();
//     println!("After calling closure: {:?}", list);
// }

use std::thread;
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

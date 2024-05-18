// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
//     // two types in x, y need to be the same
//     // let wont_work = Point { x: 5, y: 4.0 };
//     println!("Point with integer {:?}", integer);
//     println!("Point with float {:?}", float);
// }

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}
fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("Point with integer {:?}", both_integer);
    println!("Point with float {:?}", both_float);
    println!("Point with integer and float {:?}", integer_and_float);
}

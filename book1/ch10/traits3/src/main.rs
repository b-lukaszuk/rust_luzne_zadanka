// traits as parameters
// pub trait Summary {
//     fn summarize(&self) -> String;
// }
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
// fn main() {
//     println!("Hello, world!");
// }

// trait bound syntax
// pub trait Summary {
//     fn summarize(&self) -> String;
// }
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
// fn main() {
//     println!("Hello, world!");
// }

// multiple trait bounds with the + syntax
// pub trait Summary {
//     fn summarize(&self) -> String;
// }
// pub fn notify<T: Summary + std::fmt::Display>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
// fn main() {
//     println!("Hello, world!");
// }

// clearer trait bounds with where clauses
pub trait Summary {
    fn summarize(&self) -> String;
}
pub fn notify<T>(item: &T)
where
    T: Summary + std::fmt::Display,
{
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    println!("Hello, world!");
}

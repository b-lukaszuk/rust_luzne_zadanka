// returning types that implement traits
// pub trait Summary {
//     fn summarize(&self) -> String {
//         format!("Read more ...")
//     }
// }
// struct Tweet {
//     username: String,
//     content: String,
//     reply: bool,
//     retweet: bool,
// }
// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("Read tweet from @{}", self.username)
//     }
// }
// // ok, but not allowed to return Tweet or NewsArticle (other class implements Summary trait)
// fn returns_summarizable() -> impl Summary {
//     Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from("of course, as you probably already know, people"),
//         reply: false,
//         retweet: false,
//     }
// }
// fn main() {
//     println!("Hello, world!");
// }

// using traits to conditionally implement methods
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

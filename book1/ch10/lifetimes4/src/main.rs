struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn _announce_and_return_part(&self, _announcement: &str) -> &str {
        println!("Attention please: {}", _announcement);
        self.part
    }
}

// static can live for the entire duration of the program
// let s: &'static str = "I have a static lifetime.";

use std::fmt::Display;

fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("Hello, world!");
}

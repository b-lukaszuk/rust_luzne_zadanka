use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    println!("Hello, from backyard.");
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

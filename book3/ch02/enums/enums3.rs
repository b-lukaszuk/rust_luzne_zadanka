#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let x = 3;
    let y = 1;
    let a = Operations::Add;
    let s = Operations::Subtract;

    println!("Enum {:#?}.run({}, {}) = {}", a, x, y, a.run(x, y));
    println!("Enum {:#?}.run({}, {}) = {}", s, x, y, s.run(x, y));
}

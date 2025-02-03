use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    value: i32,
}

// i32 -> Number<i32>
// when defined From, then automatically we get Into
// but Into trait doesn't automatically provide From
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    let num: Number = int.into();
    println!("Creating Number<i32> using int.into(), result: {:?}", num);
    println!(
        "Creating Number<i32> using Number::from(i32), result: {:?}",
        Number::from(30)
    );
}

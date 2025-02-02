use std::any::type_name;
use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn print_type<T>(_: &T) {
    print!("{}", type_name::<T>());
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    print!("Bulit-in conversion from '{}'<", my_str);
    print_type(&my_str);
    print!("> to '{}'<", my_string);
    print_type(&my_string);
    println!(">.");

    let n1 = 30;
    let num = Number::from(n1);
    println!("Custom conversion from {} to number {:?}.", n1, num);
}

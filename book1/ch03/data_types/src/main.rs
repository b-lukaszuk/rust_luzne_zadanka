// fn main() {
//     let guess = "42".parse().expect("Not a number!"); // error no clear type
//     println!("Guess is {guess}");
// }

// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number!");
//     println!("Guess is {guess}");
// }

// fn main() {
//     let x = 2.0; // f64
//     let y: f32 = 3.0; //f32
//     println!("x is {x}, y is {y}");
// }

// fn main() {
//     let sum = 5 + 10;
//     println!("sum 5 + 10 is {sum}");
//     let difference = 95.5 - 4.3;
//     println!("difference 95.5 - 4.3 is {difference}");
//     let product = 4 * 30;
//     println!("product 4 * 30 is {product}");
//     let quotient = 56.7 / 32.2;
//     println!("quotient 56.7 / 32.2 is {quotient}");
//     let truncated = -5 / 3;
//     println!("truncated -5 / 3 is {truncated}");
//     let remainder = 43 % 5;
//     println!("remaineder 43 % 5 is {remainder}");
// }

// fn main() {
//     let t = true;
//     let f: bool = false; // with explicit type annotation
//     println!("{t} {f}");
// }

// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
//     println!("{c} {z} {heart_eyed_cat}");
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     println!("{}", tup); // error, no instance of show for tup
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let (x, y, z) = tup; // error unused variables x and z
//     println!("The value of y is: {}", y);
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     let (_, y, _) = tup;
//     println!("The value of y is: {}", y);
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;
//     println!("{}, {}, {}", five_hundred, six_point_four, one);
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     println!("{}", a); // error, no show type for a
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     println!("{:?}", a);
// }

// fn main() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5]; // explicit [type, len] declaration
//     println!("{:?}", a);
// }

// fn main() {
//     let a: [i32; 5] = [1, 2, 3, 4, 5]; // explicit [type, len] declaration
//     println!("First element of array a is: {}", a[0]);
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index]; // if index is our of a bounds, then thread panic

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn main() {
    // let guess: u32 = "42".parse().expect("Not a number!");
    // println!("Guess is {guess}");

    // let x = 2.0; // f64
    // let y: f32 = 3.0; //f32
    // println!("x is {x}, y is {y}");

    // let sum = 5 + 10;
    // println!("sum 5 + 10 is {sum}");
    // let difference = 95.5 - 4.3;
    // println!("difference 95.5 - 4.3 is {difference}");
    // let product = 4 * 30;
    // println!("product 4 * 30 is {product}");
    // let quotient = 56.7 / 32.2;
    // println!("quotient 56.7 / 32.2 is {quotient}");
    // let truncated = -5 / 3;
    // println!("truncated -5 / 3 is {truncated}");
    // let remainder = 43 % 5;
    // println!("remaineder 43 % 5 is {remainder}");

    // let t = true;
    // let f: bool = false;
    // println!("{t} {f}");

    // let c = 'z';
    // let z: char = 'Z'; // with explicity type annotation
    // let sth = '😻';
    // println!("{c} {z} {sth}");

    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("{}", tup.1);
    // println!("{x}, {y}, {z}");
    // let c = tup.0;
    // println!("tup 0 is {c}");

    // let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {}", a[0]);
}

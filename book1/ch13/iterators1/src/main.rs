fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    println!("v1_iter: {:?}", v1_iter);
    println!("iterating through v1_iter");
    // cannot use &v1_iter
    for val in v1_iter {
        println!("Got: {}", val);
    }
    // used up (borrowed elts) while iterating
    // println!("v1_iter: {:?}", v1_iter);
    println!("cannot print v1_iter (used up during iteration)");

    println!("\nv1: {:?}", v1);
    println!("iterating through v1");
    // cannot use v1 if later v1 is in println!, cause it borrows values
    for val in &v1 {
        println!("Got: {}", val);
    }
    println!("v1: {:?}", v1);
}

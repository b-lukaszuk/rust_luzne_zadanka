fn main() {
    // let s = "hello"; // string literal
    // println!("{}", s);

    // String type to be put on the heap at runtime
    // let s = String::from("hello");
    // println!("{}", s);

    // let mut s = String::from("hello");
    // s.push_str(", world!"); // appends a literal to a String
    //                         // Strings can be mutated but literals cannot
    // println!("{}", s);

    // let x = 5;
    // let y = x; // two 5s put (copied) on the stack
    // println!("{} {}", x, y);

    // let s1 = String::from("hello");
    // let s2 = s1;
    // // println!("{}", s1); // error, s2 is the owner
    // println!("{}", s2); // s1 was moved into s2

    // let s1 = String::from("hello");
    // let s2 = s1.clone(); // like (deep?)copy from other programming languages
    // println!("s1 = {}, s2 = {}", s1, s2);

    // The mechanics of passing a value to a function are similar to
    // those when assigning a value to a variable
    // let s = String::from("hello");
    // takes_ownership(s); // cannot use s after that line
    // let x = 5;
    // makes_copy(x); // can use x after that line

    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2); // cannot use s2 after that line
    // println!("{}, {}", s1, s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

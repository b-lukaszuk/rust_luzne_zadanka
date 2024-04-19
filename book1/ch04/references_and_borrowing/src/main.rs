fn main() {
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // let s = String::from("hello");
    // change(&s);

    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("{}", s);

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow mut ref a second time
    // println!("{}, {}", r1, r2);

    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here
    // let r2 = &mut s;
    // println!("{}", r2);

    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // // variables r1 and r2 will not be used after this point
    // let r3 = &mut s; // no problem
    // println!("{}", r3);

    // let reference_to_nothing = dangle();

    let reference_to_something = no_dangle();
    println!("{}", reference_to_something);
}

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // error reference is not mutable
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

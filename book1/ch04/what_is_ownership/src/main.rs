// fn read(y: bool) {
//     if y {
//         println!("y is true!");
//     }
// }

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn main() {
    // safe program
    // let x = true;
    // read(x);

    // unsafe program, using x before it's defined
    // check at compile time
    // read(x);
    // let x = true;

    // let first = String::from("Ferris");
    // let full = add_suffix(first);
    // println!("{full}");
    // cannot use
    // println!("{full}, originally {first}"); // value moved to full

    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

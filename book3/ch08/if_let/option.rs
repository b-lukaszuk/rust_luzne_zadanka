fn main() {
    // let optional: Option<i32> = None;
    let optional: Option<i32> = Some(7);

    match optional {
        Some(i) => println!("This is a really long string and `{:?}.`", i),
        _ => {} // ^ required because `match` is exhaustive, but wasted space
                // or
                // _ => println!("This is {:?}", {}),
    }
}

fn main() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("'i' is greater than 9, quit!");
            optional = None;
        } else {
            println!("'i' is '{:?}'. Try again.", i);
            optional = Some(i + 1);
        }
    }
}

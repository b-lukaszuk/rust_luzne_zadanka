fn main() {
    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("'i' is greater than 9, quit!");
                    optional = None;
                } else {
                    println!("'i' is '{:?}'. Try again.", i);
                    optional = Some(i + 1);
                }
            }
            // quit the loop when the destructure fails
            _ => {
                break;
            }
        }
    }
}

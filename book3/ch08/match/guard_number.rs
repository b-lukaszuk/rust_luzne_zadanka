fn main() {
    // let number: u8 = 4;
    let number: u8 = 0;
    // let number: i8 = -4; // will cause error declared in unreachable

    match number {
        i if i == 0 => println!("Zero."),
        i if i > 0 => println!("Greater than zero."),
        _ => unreachable!("Should never happen."),
        // TODO ^ uncomment to fix compilation
    }
}

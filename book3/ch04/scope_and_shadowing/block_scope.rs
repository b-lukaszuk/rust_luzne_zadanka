fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);
}

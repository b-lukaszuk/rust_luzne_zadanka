fn main() {
    let number = 13;
    // TODO ^ Try differetn values for 'number'

    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);
}

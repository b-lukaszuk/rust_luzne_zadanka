fn main() {
    // let triple = (0, -2, 3);
    // let triple = (5, -10, 13);
    // let triple = (3, 0, 4);
    let triple = (-3, -3, 2);
    // TODO ^ Try different values for 'triple'

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}

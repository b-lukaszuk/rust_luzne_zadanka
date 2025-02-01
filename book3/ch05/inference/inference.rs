fn main() {
    let elem = 5u8;

    // at this point the compiler knows only that we got `Vec<_>`
    let mut vec = Vec::new();

    // now the compiler knows that we got `Vec<u8>`
    vec.push(elem);
    // TODO ^ Try commenting out the `vec.push(elem)` line

    println!("{:?}", vec);
}

fn main() {
    // Try changing the values in the array, or make it a slice!
    // let array = [1, -2, 6];
    // let array = [3, -2, 6, 7]; // to work, need to comment tripple patterns
    let array = [3, -2, 6]; // can be only 3-elts (see patters from top)

    match array {
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        // single value ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),
        // the rest is ignores with ..
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // this will not compile (2 elts only)
        // [-1, second] => ...

        // match all other elements (...) in a slice named (@) `tail`
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // this expression will be assigned to `y`
        x_cube + x_squared + x
    };

    #[allow(un used_must_use)]
    let z = {
        2 * x; // statement, returns `()`
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

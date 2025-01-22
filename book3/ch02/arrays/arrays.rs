use std::mem;

// function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // fixed-size array (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // all elements initialized to the same value
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    println!("Number of elements in array: {}", xs.len());

    // arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // slices can point to a section of an array
    // [starting_index..ending_index] (inclusive..exclusive)
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1..4]);

    // Example of empty slice `&[]`:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    for i in 0..xs.len() + 1 {
        // Oops, one element too far!
        match xs.get(i) {
            // get returns Option, use pattern matching
            // or `expect()`
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // Out of bound indexing on array with constant value causes compile time error
    //println!("{}", xs[5]);
    // Out of bound indexing on slice causes runtime error
    //println!("{}", xs[..][5]);
}

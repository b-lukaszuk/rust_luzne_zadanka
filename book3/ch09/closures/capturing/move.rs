fn main() {
    // 'Vec' has non-copy semantics
    let haystack = vec![1, 2, 3];

    // `move` before vertical pipes forces closure to take ownership of captured
    // variables
    let contains = move |needle| haystack.contains(needle);
    // let contains = |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ error, borrow checker doesn't allow re-using variable after it
    // has been moved

    // removing `move` from closure's signature will cause closure
    // to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting the above line will not cause an error.
}

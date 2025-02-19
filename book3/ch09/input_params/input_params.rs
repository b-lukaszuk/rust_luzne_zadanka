// Fn - value captured by reference (&T)
// FnMut - value captured by mutable reference (&mut T)
// FnOnce - value captured by value (T)

// fn that takes a closure as arg and calls it
// 'F' - generic type parameter
fn apply<F>(f: F)
where
    // the closure takes no input and returns nothing.
    F: FnOnce(),
    // F: FnMut(),
    // F: Fn(),
{
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
}

// function that takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32
where
    // the closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // a non-copy type
    // `to_owned` creates owned data from borrowed one
    let mut farewell = "goodbye".to_owned();

    // capture 2 variables: `greeting` by reference and
    // `farewell` by value
    let diary = || {
        // `greeting` is by reference: requires `Fn`
        println!("I said {}.", greeting);

        // mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`
        mem::drop(farewell);
    };

    // call the function which applies the closure
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

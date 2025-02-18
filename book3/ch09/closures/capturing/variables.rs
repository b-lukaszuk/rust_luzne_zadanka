fn main() {
    use std::mem;

    let color = String::from("green");

    // below 'color' is borrowed ('&' - immutable borrow)
    let print = || println!("'color': {}", color);
    print();

    // color can have many immutable borrows
    let _reborrow = &color;
    print();

    // a move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // closure to increment `count` could take `&mut count` or `count`
    // `&mut count` is less restrictive so it takes it
    //
    // `mut` on `inc` because a `&mut` is stored inside. Thus,
    // calling the closure mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();

    // inc closure still mutably borrows `count` because it is called later
    // the reborrow will lead to an error
    // let _reborrow = &count;
    // ^ TODO: try uncommenting this line
    inc();

    // inc() no longer needs to borrow `&mut count` (no longer used in the code)
    // reborrow is OK here
    let _count_reborrowed = &mut count;

    // a non-copy type.
    let movable = Box::new(3);
    // `mem::drop` requires `T` so this must take by value. A copy type
    // would copy into the closure leaving the original untouched.
    // A non-copy must move and so `movable` immediately moves into
    // the closure.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // 'consume' consumes the variable so this can only be called once
    consume();
    // consume();
    // ^ TODO: Try uncommenting this line
}

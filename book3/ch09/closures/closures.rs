fn main() {
    let outer_var = 42;

    // oridinary function can't refer to vars from the enclosing env
    // fn function(i: i32) -> i32 {i + outer_var}
    // TODO: uncomment the line above and see the error

    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
    // once inferred, closure types cannot be changed
    // println!("cannot reuse closure_inferred with another type: {}", closure_inferred(42i64);
    // println!("cannot reuse closure_inferred with another type: {}", closure_inferred(3.14));
    // TODO: uncomment the line above and see the error

    let one = || 1;
    println!("closure taking no args and returning one: {}", one());
}

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32; // suffix annotation

    // defaults used
    let default_float = 3.0;
    let default_integer = 7;

    // type inferred from context (second line provides the context)
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    let mut mutable = 12;
    mutable = 21;

    // error, the type of a variable can't be changed.
    // mutable = true;

    // overwritting variable with shadowing.
    let mutable = true;

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    let my_tuple = (5u32, 1u8, true, -5.04f32);

    println!("Just some variables and types in the code. Nothing to see here.");
}

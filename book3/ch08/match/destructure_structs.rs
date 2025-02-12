fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // let foo = Foo { x: (1, 2), y: 3 };
    // let foo = Foo { x: (3, 3), y: 2 };
    let foo = Foo { x: (3, 3), y: 255 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x[0] is 1, x[1] = {},  y = {} ", b, y),
        // destructure struct, rename the variables, don't care about order
        Foo { y: 2, x: i } => println!("y is 2, x is {:?}", i),
        // ignore some variables with ...
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        //Foo { y } => println!("y = {}", y), // error: pattern does not mention field `x`
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // destructure without match block
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    // destructure with nested structs
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}

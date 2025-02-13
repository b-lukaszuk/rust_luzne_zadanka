// challenge: fix the code below

// the enum neither implements nor derives PartialEq
// that's why comparing Foo::Bar == a fails below
#[derive(PartialEq)]
enum Foo {
    Bar,
}

fn main() {
    let a = Foo::Bar;

    if Foo::Bar == a {
        // ^-- this causes a compile-time error. Use `if let` instead.
        println!("`a` is a foobar.");
    }

    // #[allow(irrefutable_let_patterns)]
    // if let Foo::Bar = a {
    //     println!("`a` is a foobar.");
    // }
}

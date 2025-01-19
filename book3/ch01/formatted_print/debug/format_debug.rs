#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!(
        "Now {:?} will print! The structure contains {}",
        Structure(3),
        Structure(3).0
    );

    println!(
        "Now {:?} will print! The Deep inside it contains {}",
        Deep(Structure(7)),
        Deep(Structure(7)).0 .0
    );
}

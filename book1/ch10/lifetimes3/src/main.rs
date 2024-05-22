// lifetime annotations in struct definitions
#[derive(Debug)]
struct ImportatntExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportatntExcerpt {
        part: first_sentence,
    };
    println!("i: {:?}", i);
    println!("i.part: {:?}", i.part);
}

// Elision rules:
// 1) compiler assigns a lifetime parameter to each parameter that's a reference
// 2) exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3) multiple input parameters but one &self | &mut self, the lifetime of self is assigned to all output lifetime parameters

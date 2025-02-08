fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    // mutably borrows each element, collection may be modified inplace
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}

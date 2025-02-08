fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // borrows each element, leaves the collection untouched
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleteing the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
}

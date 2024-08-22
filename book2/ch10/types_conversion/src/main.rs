// simple solution,
// not idiot/error-proof, not optimized for speed

fn convert_string_to_bool(text: &str) -> Option<bool> {
    match text {
        "true" | "yes" | "1" => Some(true),
        "false" | "no" | "0" => Some(false),
        _ => None,
    }
}

fn declare_conversion(text: &str) {
    match convert_string_to_bool(text) {
        Some(b) => println!("Converting '{}' to bool, result: {}.", text, b),
        None => println!("Cannot convert '{}' to bool. Conversion undefined.", text),
    };
}

fn main() {
    println!("Toy program.");
    println!("It converts strings to bools (if possible).\n");

    let tests: Vec<&str> = vec!["true", "false", "yes", "no", "1", "0", "a", "5"];
    tests.iter().for_each(|test| declare_conversion(test));

    println!("\nThat's all. Goodbye!");
}

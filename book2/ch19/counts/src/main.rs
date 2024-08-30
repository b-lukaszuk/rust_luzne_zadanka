use std::collections::HashMap;

// text should not contain too many strange characters
// only removes commas and dots from text
fn get_counts(text: String) -> HashMap<String, i32> {
    let words: Vec<String> = text
        .replace(",", "")
        .replace(".", "")
        .split_whitespace()
        .map(|word| word.to_lowercase().to_string())
        .collect();
    let mut counts: HashMap<String, i32> = HashMap::new();
    for word in words {
        if counts.contains_key(&word) {
            counts.insert(word.clone(), counts.get(&word).unwrap() + 1);
        } else {
            counts.insert(word, 1);
        }
    }
    counts
}

fn main() {
    println!("Toy program.");
    println!("It counts words in a text (case insensitive).\n");

    let text: String = String::from("a b A c d. car d, d Car");
    println!("Original text: {}", text);
    println!("Counts: {:?}", get_counts(text));

    println!("\nThat's all. Goodbye!");
}

fn starts_with_vowel(word: &str) -> bool {
    for ch in ("aeiouy").chars() {
        if word.starts_with(ch) {
            return true;
        }
    }
    false
}

fn pig_latinize_word(word: &str) -> String {
    if starts_with_vowel(word) {
        return format!("{word}-hay");
    } else {
        return format!("{}-{}ay", &word[1..], &word[0..1]);
    }
}

fn pig_latinize_text(text: &str) -> String {
    let mut result: String = String::new();
    for word in text.split_ascii_whitespace() {
        result.push_str(&pig_latinize_word(word));
        result.push_str(" ");
    }
    result
}

fn main() {
    let word1: String = String::from("apple");
    let word2: String = String::from("first");
    println!("Pig latin: '{}'", pig_latinize_word(&word1));
    println!("Original: '{}'", word1);
    println!("Pig latin: '{}'", pig_latinize_word(&word2));
    println!("Original: '{}'", word2);

    let text1: String = String::from("this is just some sentence");
    println!("Pig latin: '{}'", pig_latinize_text(&text1));
    println!("Original: '{}'", text1);
}

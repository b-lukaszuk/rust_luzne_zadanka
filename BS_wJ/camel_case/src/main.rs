fn change_to_snake_case(camel_cased_word: &String) -> String {
    let mut result: String = "".to_string();
    for c in camel_cased_word.chars() {
        if c.is_uppercase() {
            result.push('_');
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

fn change_to_camel_case(snake_cased_word: &String) -> String {
    let mut result: String = "".to_string();
    let mut prev_underscore: bool = false;
    for c in snake_cased_word.chars() {
        if c == '_' {
            prev_underscore = true;
            continue;
        } else {
            result.push(if prev_underscore {
                c.to_ascii_uppercase()
            } else {
                c
            });
            prev_underscore = false;
        }
    }
    result
}

fn print_examples(v: Vec<String>, converter: fn(&String) -> String) {
    for word in v {
        println!("{} => {}", word, converter(&word));
    }
}

fn print_intro() {
    println!("Toy program for the camel case problem.");
    println!("Warning. It may or may not work correctly.\n");
}

fn print_outro() {
    println!("\nThat's all. Goodbye!");
}

fn print_solution() {
    println!("Examples.\n");
    println!("Camel case to snake case conversion.\n");
    print_examples(
        vec![
            "helloWorld".to_string(),
            "niceToMeetYou".to_string(),
            "translateToEnglish".to_string(),
        ],
        change_to_snake_case,
    );
    println!("\nSnake case to camel case conversion.\n");
    print_examples(
        vec![
            "hello_world".to_string(),
            "nice_to_meet_you".to_string(),
            "translate_to_english".to_string(),
        ],
        change_to_camel_case,
    );
}

fn main() {
    print_intro();

    print_solution();

    print_outro();
}

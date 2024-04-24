// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear();
//     println!("First word: '{}', s: '{}'", word, s);
// }

// fn main() {
//     let s = String::from("hello world");
//     let hello = &s[0..5]; // same as &s[..5]
//     let world = &s[6..11]; // same as &s[6..]
//     let len = s.len();
//     let whole_sentence = &s[0..len]; // same as &s[..]
//     println!(
//         "{}, {}, and {}, whole sentence: {}",
//         s, hello, world, whole_sentence
//     );
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn main() {
//     let mut s = String::from("hello world");
//     let word = first_word(&s);
//     s.clear(); // error, due to the reference above
//     println!("First word: '{}', s: '{}'", word, s);
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn main() {
//     let my_string = String::from("hello world");

//     let word = first_word(&my_string[0..6]);
//     println!("{}", word);
//     let word = first_word(&my_string[..]);
//     println!("{}", word);
//     let word = first_word(&my_string);
//     println!("{}", word);

//     let my_string_literal = "hello world";

//     let word = first_word(&my_string_literal[0..6]);
//     println!("{}", word);
//     let word = first_word(&my_string_literal[..]);
//     println!("{}", word);

//     let word = first_word(my_string_literal);
//     println!("{}", word);
// }

fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    assert_eq!(slice, &[2, 3]);
    println!("All correct");
}

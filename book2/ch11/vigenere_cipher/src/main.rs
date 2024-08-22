// simple solution,
// not idiot/error-proof, not optimized for speed
// actually, it's a tweaked Caesar Cipher solution
use std::char;
use std::iter::zip;
use std::mem;

// coding (rotate_by > 0) and decoding (rotate_by < 0) by substitution
fn code_letter(letter: char, rotate_by: i32) -> char {
    let mut alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz");
    if letter.is_uppercase() {
        alphabet = alphabet.to_uppercase();
    }
    let rot: i32 = rotate_by.abs() % alphabet.len() as i32;
    let mut rotated_alphabet: String = format!(
        "{}{}",
        &alphabet[rot as usize..alphabet.len()],
        &alphabet[..rot as usize]
    );
    // decoding
    if rotate_by < 0 {
        mem::swap(&mut alphabet, &mut rotated_alphabet);
    }
    match alphabet.find(letter) {
        Some(index) => return rotated_alphabet.chars().nth(index).unwrap(),
        None => return letter,
    }
}

fn get_rotation(keyword_letter: char) -> i32 {
    let alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz");
    match alphabet.find(keyword_letter) {
        Some(index) => return index as i32,
        None => return -1,
    }
}

fn code_msg(msg: String, keyword: String, decode_mode: bool) -> String {
    let rot_direction = if decode_mode { -1 } else { 1 };
    let mut kwrd: String = keyword;
    // multiply kwrd, so len(kwrd) >= len(msg)
    if kwrd.len() < msg.len() {
        let ratio: f64 = msg.len() as f64 / kwrd.len() as f64 + 1.0;
        kwrd = kwrd.repeat(ratio.ceil() as usize);
    }
    let rotations: Vec<i32> = kwrd
        .chars()
        .map(|k| get_rotation(k) * rot_direction)
        .collect();
    let zipped = zip(msg.chars(), rotations);
    zipped.map(|(c, rot)| code_letter(c, rot)).collect()
}

fn main() {
    println!("Toy program.\n");

    let mut msg: String = String::from("aaa");
    let mut keyword: String = String::from("bcd");
    let mut coded_msg: String = code_msg(msg.clone(), keyword.clone(), false);
    println!(
        "'{}' coded with '{}' keyword is: '{}'",
        msg, keyword, coded_msg
    );

    msg = String::from("abc");
    coded_msg = code_msg(msg.clone(), keyword.clone(), false);
    println!(
        "'{}' coded with '{}' keyword is: '{}'",
        msg, keyword, coded_msg
    );

    msg = String::from("I came, I saw, I conquered");
    keyword = String::from("rusty");
    coded_msg = code_msg(msg.clone(), keyword.clone(), false);
    println!(
        "'{}' coded with '{}' keyword is: '{}'",
        msg, keyword, coded_msg
    );
    println!(
        "'{}' decoded with '{}' keyword is: '{}'",
        coded_msg,
        keyword,
        code_msg(coded_msg.clone(), keyword.clone(), true)
    );

    println!("\nThat's all. Goodbye!");
}

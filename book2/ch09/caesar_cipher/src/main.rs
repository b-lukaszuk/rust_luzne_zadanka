// simple solution,
// not idiot/error-proof, not optimized for speed

use std::char;
use std::mem;

fn code_letter(letter: char, rotate_by: i32) -> char {
    let mut alphabet: String = String::from("abcdefghijklmnopqrstuvwxyz");
    if letter.is_uppercase() {
        alphabet = alphabet.to_uppercase();
    }
    let rot: i32 = rotate_by.abs();
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

fn code_msg(msg: String, rotate_by: i32) -> String {
    msg.chars().map(|c| code_letter(c, rotate_by)).collect()
}

fn main() {
    println!("Toy program.\n");

    let msg: String = String::from("L fdph, L vdz, L frqtxhuhg.");
    let rot: i32 = -3;

    println!("'{}' decoded with rot {} is:", msg, rot);
    println!("'{}'", code_msg(msg, rot));

    println!("\nThat's all. Goodbye!");
}

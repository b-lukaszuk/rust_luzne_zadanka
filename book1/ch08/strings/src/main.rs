// fn main() {
//     let s = String::new();
//     println!("s: '{}'", s);
// }

// fn main() {
//     let data = "initial contents";
//     let s = data.to_string();

//     // the method also works on a literal directly
//     let s = "initial contents".to_string();

//     let s = String::from("initial contents");
// }

// fn main() {
//     let hello = String::from("السلام عليكم");
//     let hello = String::from("Dobrý den");
//     let hello = String::from("Hello");
//     let hello = String::from("שָׁלוֹם");
//     let hello = String::from("नमस्ते");
//     let hello = String::from("こんにちは");
//     let hello = String::from("안녕하세요");
//     let hello = String::from("你好");
//     let hello = String::from("Olá");
//     let hello = String::from("Здравствуйте");
//     let hello = String::from("Hola");
// }

// fn main() {
//     let mut s1 = String::from("foo");
//     let s2 = "bar";
//     s1.push_str(s2); // it does not take ownership of s
//     println!("s1: {}, s2: {}", s1, s2);
// }

// fn main() {
//     let mut s1 = String::from("foo");
//     let s2 = String::from("bar");
//     s1.push_str(&s2);
//     println!("s1: {}, s2: {}", s1, s2);
// }

fn main() {
    let mut s1 = String::from("foo");
    let mut s2 = String::from("foo");
    s1.push('b');
    s2.push_str("bar");
    println!("s1: {}, s2: {}", s1, s2);
}

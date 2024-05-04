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

// fn main() {
//     let mut s1 = String::from("foo");
//     let mut s2 = String::from("foo");
//     s1.push('b');
//     s2.push_str("bar");
//     println!("s1: {}, s2: {}", s1, s2);
// }

// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("world!");
//     // + internally uses: add(self, s: &str) -> String
//     let s3 = s1 + &s2; // string concatenation, s1 is moved here
//     println!("s3: {}", s3);
// }

// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     let s = s1 + "-" + &s2 + "-" + &s3;

//     println!("s: {}", s);
// }

// fn main() {
//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     // neither s1, s2, nor s3 is moved
//     let s = format!("{s1}-{s2}-{s3}");

//     println!("s: {}", s);
// }

// fn main() {
//     let s = String::from("hello");
//     // indexing of strings is not possible in Rust
//     let h = s[0];
//     println!("s: {}", s);
//     println!("h: {}", h);
// }

// fn main() {
//     let hello = "Здравствуйте";
//     let s = &hello[0..4]; // slices are ok
//     println!("s: {}", s);
//     println!("hello: {}", hello);
// }

fn main() {
    let hello = "Здравствуйте";
    for c in hello.chars() {
        println!("{c}");
    }
    for b in hello.bytes() {
        println!("{b}");
    }
}

// fn main() {// inifite loop,
//     loop {// stop it with Ctrl+C (even pressed a few times) in terminal
//         println!("again");
//     }
// }

// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The result is {result}");
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// fn main() {
//     let mut number = 3;
//     while number != 0 {
//         println!("{number}!");
//         number -= 1;
//     }
//     println!("LIFTOFF!!!");
// }

// fn main() {
//     let arr = [10, 20, 30, 40, 50];
//     let mut index = 0;
//     while index < 5 {
//         println!("the value is: {}", arr[index]);
//         index += 1;
//     }
// }

// fn main() {
//     let arr = [10, 20, 30, 40, 50];
//     for elt in arr {
//         println!("the value is: {elt}");
//     }
// }

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

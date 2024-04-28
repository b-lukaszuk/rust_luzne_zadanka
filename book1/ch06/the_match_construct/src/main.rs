// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     let c = Coin::Nickel;
//     println!("A nickel is worth {} cents", value_in_cents(c));
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// fn main() {
//     let nickel = Coin::Nickel;
//     let penny = Coin::Penny;
//     println!("A nickel is worth {} cents", value_in_cents(nickel));
//     println!("A penny is worth {} cents", value_in_cents(penny));
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//         None => None,
//     }
// }

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("{:?}", five);
//     println!("{:?}", six);
//     println!("{:?}", none);
// }

// fn main() {
//     let dice_roll = 9;
//     match dice_roll {
//         3 => println!("Add fancy hat."),
//         7 => println!("remove fancy hat."),
//         _ => println!("move player."),
//     }
// }

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("Add fancy hat."),
        7 => println!("remove fancy hat."),
        _ => (),
    }
}

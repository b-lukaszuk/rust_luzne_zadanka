// fn main() {
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {}", max),
//         _ => (),
//     }
// }

// fn main() {
//     let config_max = Some(3u8);
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {}", max);
//     }
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(String),
// }

// fn main()
//     let mut count = 0;
//     let coin = Coin::Dime;
//     // let coin = Coin::Quarter(String::from("XX"));
//     match coin {
//         Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//         _ => count += 1,
//     }
//     println!("Count: {}", count)
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

fn main() {
    let mut count = 0;
    // let coin = Coin::Dime;
    let coin = Coin::Quarter(String::from("XX"));
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("Count: {}", count)
}

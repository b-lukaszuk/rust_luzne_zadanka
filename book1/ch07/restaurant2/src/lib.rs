// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("Adding to waitlist.");
//         }
//     }
// }

// use crate::front_of_house::hosting; // idiomatic way

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("Adding to waitlist.");
//         }
//     }
// }

// use crate::front_of_house::hosting;

// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist(); // error, use not in a local scope
//     }
// }

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // snip
//     fmt::Result::Ok(())
// }

// fn function2() -> io::Result<()> {
//     io::Result::Ok(())
// }

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     Result::Ok(())
// }

// fn function2() -> IoResult<()> {
//     IoResult::Ok(())
// }
//

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// use std::cmp::Ordering;
// use std::io;
//
// use std::{cmp::Ordering, io};
//
// use std::io;
// use std::io::Write;
//
// use std::io::{self, Write}
//
// use std::collections::*;

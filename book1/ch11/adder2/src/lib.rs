pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// pub fn add_two(a: i32) -> i32 {
//     a + 2
// }

pub fn add_two(a: i32) -> i32 {
    a + 3
}

// pub fn greeting(name: &str) -> String {
//     format!("Hello {}!", name)
// }

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }
//         Guess { value }
//     }
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!("Guess value must be between 1 and 100, got {}.", value);
//         }
//         Guess { value }
//     }
// }

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(result.contains("Carol"));
    // }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not caontain name value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    #[ignore] // ignores the test by default
    fn expensive_test() {
        assert_eq!(2 * 2, 4);
    }
}

// cargo test // runs all tests
// cargo test one_hundered // runs only test one_hundered
// cargo test add // runs all tests with add in the name
// cargo test -- --ignored // runs only the ignored tests

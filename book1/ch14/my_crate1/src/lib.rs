//! # My Crate
//!
//! `my_crate1` is a colelction of utilities to make performing certain
//! calculations more convenient

/// Adds one to the number given
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = my_crate1::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Adds two usize numbers together
///
/// # Examples
/// ```
/// let arg1 = 5;
/// let arg2 = 4;
/// let answer = my_crate1::add(arg1, arg2);
///
/// assert_eq!(9, answer);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(n: i32) -> i32 {
    n + 2
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

// Iterator Trait defined in the standard library
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
//     // methods with default implementations elided
// }
//
fn main() {
    println!("Hello, world!");
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn map_demonstration() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // or v2: Vec<i32>
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

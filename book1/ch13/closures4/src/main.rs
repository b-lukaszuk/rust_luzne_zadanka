// default std library unwrap_or_else implementation
// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T,
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }
//
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];
//     list.sort_by_key(|r| r.width);
//     println!("{:#?}", list);
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

// fn main() {
//     let v: Vec<i32> = Vec::new();
//     println!("v: {:?}", v);
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     println!("v: {:?}", v);
// }

// fn main() {
//     let mut v = Vec::new();
//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);
//     println!("v: {:?}", v);
// }

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element is {third}");

//     let third: Option<&i32> = v.get(2);
//     match third {
//         Some(elt) => println!("The third elementt is {elt}"),
//         None => println!("There is no third element"),
//     }

//     // let does_not_exist = &v[100]; // thread panic
//     let does_not_exist = v.get(100);
//     println!("Accessing element that does not exist {:?}", does_not_exist);
// }

// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];

//     let first = &v[0]; // immutable borrow

//     v.push(6); // mutable borrow

//     println!("The first element is {first}");
// }

// fn main() {
//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("{i}");
//     }
// }

// fn main() {
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//         println!("{i}");
//     }
// }

fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here
}

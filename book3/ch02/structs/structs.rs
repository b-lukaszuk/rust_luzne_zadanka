#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit struct
struct Unit;

// tuple struct
struct Pair(i32, f32);

// struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn get_area(r: Rectangle) -> f32 {
    let Rectangle {
        top_left: tl,
        bottom_right: br,
    } = r;
    let height = (tl.y - br.y).abs();
    let width = (br.x - tl.x).abs();
    height * width
}

fn get_square(top_left: Point, side_len: f32) -> Rectangle {
    let bottom_right: Point = Point {
        x: top_left.x + side_len,
        y: top_left.y - side_len,
    };
    Rectangle {
        top_left,
        bottom_right,
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    // struct update syntax
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // destructuring with a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };
    println!("\n{:#?}", rectangle);
    println!("Area: {:.2} [cm^2]\n", get_area(rectangle));

    let pt = Point { x: 0.0, y: 0.0 };
    let side_len = 10.0;
    let rct = get_square(pt, side_len);
    println!(
        "\nSquare of rect struct with side len = {}.\n{:#?}\n",
        side_len, rct
    );

    // instantiate a unit struct
    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

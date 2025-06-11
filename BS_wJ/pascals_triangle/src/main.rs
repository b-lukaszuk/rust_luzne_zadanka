fn get_sum_of_pairs(row: &Vec<i32>) -> Vec<i32> {
    assert!(row.len() > 1);
    let mut result: Vec<i32> = vec![];
    for i in 0..row.len() - 1 {
        result.push(row[i] + row[i + 1]);
    }
    result
}

fn get_row(prev_row: &Vec<i32>) -> Vec<i32> {
    assert!(prev_row.len() > 1);
    let mut result: Vec<i32> = get_sum_of_pairs(prev_row);
    result.insert(0, 1);
    result.push(1);
    result
}

fn get_pascal_triangle(n: u32) -> Vec<Vec<i32>> {
    assert!(0 < n && n < 11);
    let mut triangle: Vec<Vec<i32>> = vec![vec![1], vec![1, 1]];
    for i in 2..=n as usize {
        triangle.push(get_row(&triangle[i - 1]));
    }
    triangle
}

fn print_pasc_traingle(n: u32) {
    assert!(0 < n && n < 11);
    let triangle: Vec<Vec<i32>> = get_pascal_triangle(n);
    for r in triangle {
        println!("{:?}", r);
    }
}

fn print_intro() {
    println!("Toy program.");
    println!("It displays Pascal's triangles (it may or may not work correctly).");
}

fn print_solution() {
    println!("\nThe solution goes here.");
}

fn print_outro() {
    println!("\nThat's all. Goodbye!");
}

fn main() {
    print_intro();

    print_solution();
    print_pasc_traingle(3);

    print_outro();
}

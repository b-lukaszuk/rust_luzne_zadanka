fn get_sum_of_pairs(row: &Vec<u32>) -> Vec<u32> {
    assert!(row.len() > 1);
    let mut result: Vec<u32> = vec![];
    for i in 0..row.len() - 1 {
        result.push(row[i] + row[i + 1]);
    }
    result
}

fn get_row(prev_row: &Vec<u32>) -> Vec<u32> {
    assert!(prev_row.len() > 1);
    let mut result: Vec<u32> = get_sum_of_pairs(prev_row);
    result.insert(0, 1);
    result.push(1);
    result
}

fn get_pascal_triangle(n: u32) -> Vec<Vec<u32>> {
    assert!(0 < n && n < 11);
    let mut triangle: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];
    for i in 2..=n as usize {
        triangle.push(get_row(&triangle[i - 1]));
    }
    triangle
}

fn get_num_len(n: u32) -> usize {
    n.to_string().len()
}

fn get_max_num_len(v: &Vec<u32>) -> usize {
    v.iter().map(|n| get_num_len(*n)).max().unwrap()
}

fn print_pasc_traingle(n: u32) {
    assert!(0 < n && n < 11);
    let triangle: Vec<Vec<u32>> = get_pascal_triangle(n);
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

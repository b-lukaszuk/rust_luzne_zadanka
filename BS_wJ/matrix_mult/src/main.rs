fn get_n_rows(matrix: &Vec<Vec<i32>>) -> usize {
    matrix.len()
}

fn get_n_cols(matrix: &Vec<Vec<i32>>) -> usize {
    matrix[0].len()
}

fn get_col(matrix: &Vec<Vec<i32>>, col_ind: usize) -> Vec<i32> {
    let n_rows: usize = get_n_rows(matrix);
    let mut col: Vec<i32> = vec![];
    for i in 0..n_rows {
        col.push(matrix[i][col_ind]);
    }
    col
}

fn get_dot_prod(row: &Vec<i32>, col: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for i in 0..row.len() {
        result.push(row[i] * col[i])
    }
    result
}

fn mult(matrix1: &Vec<Vec<i32>>, matrix2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![];
    for r in 0..get_n_rows(matrix1) {
        result.push(vec![]);
        for c in 0..get_n_cols(matrix2) {
            result[r].push(get_dot_prod(&matrix1[r], &get_col(matrix2, c)).iter().sum());
        }
    }
    result
}

fn my_print(matrix: &Vec<Vec<i32>>) {
    for r in 0..matrix.len() {
        for c in 0..get_n_cols(matrix) {
            print!("{} ", matrix[r][c]);
        }
        println!("");
    }
}

fn main() {
    println!("Toy program for multiplying matrices.");
    println!("Warning. It may or may not work correctly.\n");

    let a: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let b: Vec<Vec<i32>> = vec![vec![7, 8], vec![9, 10], vec![11, 12]];
    my_print(&a);
    println!("multiplied by");
    my_print(&b);
    println!("gives");
    my_print(&mult(&a, &b));

    println!("");

    let c: Vec<Vec<i32>> = vec![vec![-1, 3, 5], vec![5, 5, 2]];
    let d: Vec<Vec<i32>> = vec![vec![3, 4], vec![3, -2], vec![4, -2]];
    my_print(&c);
    println!("multiplied by");
    my_print(&d);
    println!("gives");
    my_print(&mult(&c, &d));

    println!("\nThat's all. Goodbye!");
}

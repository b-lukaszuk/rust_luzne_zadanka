fn get_n_rows(matrix: &Vec<Vec<f64>>) -> usize {
    matrix.len()
}

fn get_n_cols(matrix: &Vec<Vec<f64>>) -> usize {
    matrix[0].len()
}

fn get_col(matrix: &Vec<Vec<f64>>, col_ind: usize) -> Vec<f64> {
    let n_rows: usize = get_n_rows(matrix);
    let mut col: Vec<f64> = vec![];
    for i in 0..n_rows {
        col.push(matrix[i][col_ind]);
    }
    col
}

fn get_dot_prod(row: &Vec<f64>, col: &Vec<f64>) -> Vec<f64> {
    let mut result: Vec<f64> = vec![];
    for i in 0..row.len() {
        result.push(row[i] * col[i])
    }
    result
}

fn mult(matrix1: &Vec<Vec<f64>>, matrix2: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let mut result: Vec<Vec<f64>> = vec![];
    for r in 0..get_n_rows(matrix1) {
        result.push(vec![]);
        for c in 0..get_n_cols(matrix2) {
            result[r].push(get_dot_prod(&matrix1[r], &get_col(matrix2, c)).iter().sum());
        }
    }
    result
}

fn my_print(matrix: &Vec<Vec<f64>>) {
    for r in 0..matrix.len() {
        for c in 0..get_n_cols(matrix) {
            print!("{} ", matrix[r][c]);
        }
        println!("");
    }
}

fn get_det2x2(matrix: &Vec<Vec<f64>>) -> f64 {
    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
}

fn get_det2x2_reciprocal(matrix: &Vec<Vec<f64>>) -> f64 {
    1.0 / get_det2x2(matrix)
}

fn get_inv2x2(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let det_rec: f64 = get_det2x2_reciprocal(matrix);
    let swap: Vec<Vec<f64>> = get_swap(matrix);
    let mut result: Vec<Vec<f64>> = vec![];
    for r in 0..get_n_rows(&swap) {
        result.push(vec![]);
        for c in 0..get_n_cols(&swap) {
            result[r].push(det_rec * swap[r][c]);
        }
    }
    result
}

fn get_swap(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let a: f64 = matrix[1][1];
    let b: f64 = matrix[0][1] * -1.0;
    let c: f64 = matrix[1][0] * -1.0;
    let d: f64 = matrix[0][0];
    vec![vec![a, b], vec![c, d]]
}

fn print_intro() {
    println!("Toy program for the bat and ball problem.");
    println!("Warning. It may or may not work correctly.\n");
}

fn print_outro() {
    println!("\nThat's all. Goodbye!");
}

fn print_problem_description() {
    println!("Bat and ball together cost 1 dollar.");
    println!("The bat costs 1 dollar more than the ball");
    println!("What is the price of the ball?\n");
}

fn print_solution() {
    let a: Vec<Vec<f64>> = vec![vec![1.0, 1.0], vec![1.0, -1.0]];
    let b: Vec<Vec<f64>> = vec![vec![1.1], vec![1.0]];
    println!("inverse of:");
    my_print(&a);
    println!("multiplied by");
    my_print(&b);
    let result: Vec<Vec<f64>> = mult(&get_inv2x2(&a), &b);
    println!(
        "tells us that, the bat = ${:.2}, the ball = ${:.2}",
        result[0][0], result[1][0]
    );
}

fn main() {
    print_intro();

    print_problem_description();
    print_solution();

    print_outro();
}

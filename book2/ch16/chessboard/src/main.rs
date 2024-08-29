// simple solution,
// not idiot/error-proof, not optimized for speed

fn get_empty_board() -> Vec<Vec<String>> {
    vec![vec![String::from(""); 8]; 8]
}

fn add_pieces_to_row(board: &mut Vec<Vec<String>>, pieces: Vec<String>, row: usize) {
    for i in 0..pieces.len() {
        board[row][i] = pieces[i].clone();
    }
}

fn get_square_to_print(symbol: String, black: bool) -> String {
    let mut fill: String = String::from(" ");
    let mut new_symbol: String = symbol.clone();
    if black {
        fill = String::from(".");
    }
    if symbol.eq("") {
        new_symbol = fill.clone();
    }
    format!("{}{}{}", fill, new_symbol, fill)
}

fn get_row_to_print(row: &Vec<String>, black: bool) -> String {
    let mut row_to_print = String::from("");
    let mut is_black: bool = black.clone();
    for col in row {
        row_to_print = format!(
            "{}| {} ",
            row_to_print,
            get_square_to_print(col.to_string(), is_black)
        );
        is_black = !is_black;
    }
    format!("{}|", row_to_print)
}

fn get_board_to_print(board: &Vec<Vec<String>>) -> String {
    let mut board_to_print: String = String::from("");
    let mut start_black: bool = false;
    for row in board {
        board_to_print = format!(
            "{}{}",
            board_to_print, "-------------------------------------------------\n"
        );
        board_to_print = format!("{}{}\n", board_to_print, get_row_to_print(row, start_black));
        start_black = !start_black;
    }
    format!(
        "{}{}",
        board_to_print, "-------------------------------------------------\n"
    )
}

fn print_board(board: &Vec<Vec<String>>) {
    println!("{}", get_board_to_print(board));
}

fn split_to_chars(text: String) -> Vec<String> {
    text.chars().map(|c| c.to_string()).collect()
}

fn main() {
    println!("Toy program.");
    println!("It displays a chessboard (starting position).\n");

    let mut board: Vec<Vec<String>> = get_empty_board();
    add_pieces_to_row(&mut board, vec![String::from("p"); 8], 1);
    add_pieces_to_row(&mut board, split_to_chars(String::from("rnbqkbnr")), 0);
    add_pieces_to_row(&mut board, vec![String::from("P"); 8], 6);
    add_pieces_to_row(&mut board, split_to_chars(String::from("RNBQKBNR")), 7);
    print_board(&board);

    println!("\nThat's all. Goodbye!");
}

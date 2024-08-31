// simple solution,
// not idiot/error-proof, not optimized for speed
use rand::Rng;
use std::io::{self, Write};
use std::{thread, time::Duration};

const WIDTH: usize = 80;
const HEIGHT: usize = 15;
const PROB_ALIVE: f64 = 0.25;
const ALIVE_SYMBOL: &str = "O";
const DEAD_SYMBOL: &str = ".";
const DELAY_MS: u64 = 500;
const NUM_CYCLES: i32 = 50;

// num_rows = HEIGHT, num_cols = WIDTH
// indexing: universe[row][col]
type Universe = [[bool; WIDTH]; HEIGHT];

fn get_empty_universe() -> Universe {
    [[false; WIDTH]; HEIGHT]
}

fn get_rand_num(min_incl: f64, max_incl: f64) -> f64 {
    rand::thread_rng().gen_range(min_incl..=max_incl)
}

fn get_rand_dead_or_alive() -> bool {
    if get_rand_num(0.0, 1.0) <= PROB_ALIVE {
        return true;
    }
    return false;
}

fn get_rand_universe() -> Universe {
    let mut universe: Universe = get_empty_universe();
    for r in 1..HEIGHT {
        for c in 1..WIDTH {
            universe[r][c] = get_rand_dead_or_alive();
        }
    }
    universe
}

fn get_field_symbol(field: bool) -> &'static str {
    if field {
        return ALIVE_SYMBOL;
    }
    return DEAD_SYMBOL;
}

fn print_universe(u: &Universe, cycle_num: u32) {
    let mut population: u32 = 0;
    for r in 1..HEIGHT {
        for c in 1..WIDTH {
            print!("{}", get_field_symbol(u[r][c]));
            if u[r][c] {
                population += 1;
            }
        }
        println!("");
    }
    println!("cycle: {}, population: {}", cycle_num, population);
}

// the terminal must support ANSI escape codes
// https://en.wikipedia.org/wiki/ANSI_escape_code
fn clean_printout() {
    // "\033[xxxxA" - xxx moves cursor up xxxx lines
    // in rust you need to use hex code instead of octal, hence "\033" is "\x1b"
    print!("\x1b[{}A", HEIGHT);
    print!("\x1b[J"); // clears from cursor position till end of display
}

fn reprint(u: &Universe, cycle_num: u32) {
    clean_printout();
    print_universe(u, cycle_num);
}

fn get_next_state(u: &Universe) -> Universe {
    let mut new_universe: Universe = get_empty_universe();
    for r in 1..HEIGHT {
        for c in 1..WIDTH {
            new_universe[r][c] = should_cell_be_alive(u, r as i32, c as i32);
        }
    }
    new_universe
}

fn is_within_range(num: i32, min_incl: i32, max_excl: i32) -> bool {
    (num >= min_incl) && (num < max_excl)
}

fn is_cell_within_range(x: i32, y: i32) -> bool {
    is_within_range(x, 0, HEIGHT as i32) && is_within_range(y, 0, WIDTH as i32)
}

fn get_num_live_neighbours(u: &Universe, x: i32, y: i32) -> i32 {
    if !is_cell_within_range(x, y) {
        return 0;
    }
    let mut count: i32 = 0;
    for r in -1..=1 {
        for c in -1..=1 {
            let (xi, yi) = (x + r, y + c);
            if (xi == x && yi == y) || !is_cell_within_range(xi, yi) {
                continue;
            }
            if u[xi as usize][yi as usize] {
                count += 1;
            }
        }
    }
    count
}

fn should_cell_be_alive(u: &Universe, x: i32, y: i32) -> bool {
    let num_live_neighbours: i32 = get_num_live_neighbours(u, x, y);
    if u[x as usize][y as usize] {
        return is_within_range(num_live_neighbours, 2, 4);
    }
    return num_live_neighbours == 3;
}

fn run_game_of_life() {
    let mut u: Universe = get_rand_universe();
    let mut cur_cycle: u32 = 0;
    print_universe(&u, cur_cycle);

    for _i in 1..=NUM_CYCLES {
        thread::sleep(Duration::from_millis(DELAY_MS));
        u = get_next_state(&u);
        cur_cycle += 1;
        reprint(&u, cur_cycle);
    }
}

fn main() {
    println!("Toy program.");
    println!("It displays a so called game of life.");
    println!("Note: your terminal must support ANSI escape codes.\n");

    let mut user_input = String::new(); // start game of life animation on keypress

    println!("The game will run through {} cycles.", NUM_CYCLES);
    println!("You can abort it any time by pressing Ctrl+C.");
    println!("Press Enter to begin.");

    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    run_game_of_life();

    println!("\nThat's all. Goodbye!\n");
}

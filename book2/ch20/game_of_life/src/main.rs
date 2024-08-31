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

fn get_next_state(_u: &Universe) -> Universe {
    get_rand_universe()
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

    println!("\nThat's all. Goodbye!\n")
}

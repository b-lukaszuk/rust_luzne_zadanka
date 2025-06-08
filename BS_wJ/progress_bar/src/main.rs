use std::{io, io::stdin, thread, time};

fn get_progress_bar(perc: u32) -> String {
    assert!(perc <= 100);
    let max_n_chars: f64 = 50.0;
    let p: u32 = (perc as f64 / (100.0 / max_n_chars)).round() as u32;
    format!(
        "{}{}{}%",
        "|".repeat(p as usize),
        ".".repeat((max_n_chars as u32 - p) as usize),
        perc
    )
}

fn animate_progress_bar() {
    let fans: Vec<&str> = vec!["\\", "-", "/", "-"];
    let mut ind: usize = 0;
    let delay_ms: time::Duration = time::Duration::from_millis(150);
    for p in 0..=100 {
        println!("{}{}", get_progress_bar(p), fans[ind]);
        thread::sleep(delay_ms);
        clear_printout();
        ind = if ind == fans.len() - 1 { 0 } else { ind + 1 }
    }
    println!("{}", get_progress_bar(100))
}

// the terminal must support ANSI escape codes
// https://en.wikipedia.org/wiki/ANSI_escape_code
fn clear_printout() {
    // "\033[xxxA" - xxx moves cursor up xxx lines
    // in rust you need to use hex code instead of octal, hence "\033" is "\x1b"
    print!("\x1b[{}A", 1);
    print!("\x1b[J"); // clears from cursor position till end of display
}

fn print_intro() {
    println!("Toy program.");
    println!("It animates a progress bar (it may or may not work correctly).");
    println!("Note: your terminal must support ANSI escape codes.\n");
}

fn print_solution() -> io::Result<()> {
    println!("Continue with the animation? [Y/n]");
    let mut choice = String::new();
    stdin().read_line(&mut choice)?;
    if vec!["y".to_string(), "yes".to_string(), "".to_string()]
        .contains(&choice.trim().to_lowercase())
    {
        animate_progress_bar();
    }

    Ok(())
}

fn print_outro() {
    println!("\nThat's all. Goodbye!");
}

fn main() {
    print_intro();

    let _ = print_solution();

    print_outro();
}

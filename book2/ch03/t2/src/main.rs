use rand::Rng;
use std::io;

fn print_intro() {
    println!("===");
    println!("Hello, let's play a game.");
    println!("I'm thinking of a number between 1 and 100.");
    println!("Can you guess it?");
    println!("To make it easier after each guess I will give you a hint");
    println!("by saying: 'too high' or 'too low'");
    println!("Ready, let's start.");
    println!("BTW. You should be able to abort ");
    println!("the game by pressing Ctrl+C (perhaps twice) on your keyboard.");
    println!("===\n");
}

fn print_outro() {
    println!("\n===");
    println!("Game over. You won. Congratulations!");
    println!("===");
}

fn get_rand_num_1_to_100() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn get_user_guess() -> u32 {
    let mut guess = String::new();
    println!("Please type an integer between 1 and 100 and press Enter");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Error, incorrect input");
    guess
}

fn print_hint(secret_num: u32, guess: u32) {
    if secret_num < guess {
        println!("Too high.")
    } else if secret_num > guess {
        println!("Too low.")
    } else {
        println!("You got it.")
    }
}

fn run_game() {
    let secret_num: u32 = get_rand_num_1_to_100();
    let mut guess: u32 = 0;

    while secret_num != guess {
        guess = get_user_guess();
        print_hint(secret_num, guess)
    }
}

fn main() {
    print_intro();
    run_game();
    print_outro();
}

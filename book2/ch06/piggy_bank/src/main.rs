use rand::Rng;

fn get_rand_num(min_incl: u32, max_incl: u32) -> u32 {
    rand::thread_rng().gen_range(min_incl..=max_incl)
}

// in cents
fn get_rand_coin() -> u32 {
    let choice: u32 = get_rand_num(1, 3);
    match choice {
        1 => 5,
        2 => 10,
        _ => 25,
    }
}

fn main() {
    let mut piggy_bank: u32 = 0; // in cents
    const UPPER_LIMIT: u32 = 20 * 10; // in cents, changed to $2 instead of $20

    println!("Toy program.\nIt adds coins at random to a piggy bank.\n");
    println!("Initial balance: ${:.2}.", (piggy_bank as f64));
    println!("Target balance: >= ${:.2}.\n", (UPPER_LIMIT as f64) / 100.0);

    while piggy_bank < UPPER_LIMIT {
        let cur_coin: u32 = get_rand_coin();
        piggy_bank += cur_coin;
        println!("Adding {} to the balance", cur_coin);
        println!(
            "Piggy bank. Current balance {:.2}",
            (piggy_bank as f64) / 100.0
        );
    }

    println!("Piggy bank. Target balance reached/exceeded.\n");
    println!("That's all. Goodbye!\n");
}

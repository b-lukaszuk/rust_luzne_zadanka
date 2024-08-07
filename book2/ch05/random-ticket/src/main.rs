use rand::Rng;

fn get_rand_num(min_incl: u32, max_incl: u32) -> u32 {
    rand::thread_rng().gen_range(min_incl..=max_incl)
}

// without enums
fn get_rand_spaceline() -> String {
    let rand_choice = get_rand_num(1, 3);
    match rand_choice {
        1 => String::from("Space Adventures"),
        2 => String::from("SpaceX"),
        _ => String::from("Virgin Galactic"),
    }
}

// in mln usd
fn get_rand_base_price() -> u32 {
    get_rand_num(36, 50)
}

// in km/sec
fn get_rand_speed() -> u32 {
    get_rand_num(16, 30)
}

fn get_time_in_days(speed_km_per_sec: u32) -> u32 {
    const DIST_TO_MARS: f64 = 62.1e6; //  km from Earth
    const SECS_PER_DAY: f64 = 60.0 * 60.0 * 24.0;
    (DIST_TO_MARS / speed_km_per_sec as f64 / SECS_PER_DAY).ceil() as u32
}

// without enums
fn get_rand_trip_type() -> String {
    let rand_choice = get_rand_num(1, 2);
    match rand_choice {
        1 => String::from("One-way"),
        _ => String::from("Round-trip"),
    }
}

fn get_total_price(base_price: u32, days: u32, trip_type: String) -> u32 {
    // (1 + 1-days/100) because faster ships are more expensive
    let speed_multiplier: f64 = 1.0 + 1.0 - ((days as f64) / 100.0);
    let mut total_price: f64 = (base_price as f64) * speed_multiplier;
    if trip_type.contains("Round-trip") {
        total_price = total_price * 2.0;
    }
    total_price.ceil() as u32
}

fn get_header() -> String {
    format!("Spaceline           Days    Trip type       Price  \n=================================================")
}

fn get_rand_ticket() -> String {
    let spaceline: String = format!("{:<20}", get_rand_spaceline());
    let days_num: u32 = get_time_in_days(get_rand_speed());
    let days_str: String = format!("{:>4}{:>4}", days_num, "");
    let trip_type: String = format!("{:<16}", get_rand_trip_type());
    let base_price: u32 = get_rand_base_price();
    let total_price: String = format!(
        "{:>5}",
        get_total_price(base_price, days_num, trip_type.clone())
    );
    format!("{}{}{}{}", spaceline, days_str, trip_type, total_price)
}

fn main() {
    println!("Printing 10 random fake flights to Mars\n");
    println!("{}", get_header());
    for _ in 1..10 {
        println!("{}", get_rand_ticket());
    }
    println!("\nThat's all. Goodbye!");
}

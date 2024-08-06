use rand::Rng;

// literal translation of wikipedia's:
// "This extra leap day occurs in each year that is a multiple of 4, except for
// years evenly divisible by 100 but not by 400."
//
fn is_x_div_by_n(x: u32, n: u32) -> bool {
    x % n == 0
}

fn is_div_by_4(x: u32) -> bool {
    is_x_div_by_n(x, 4)
}

fn is_div_by_100_but_not_400(x: u32) -> bool {
    is_x_div_by_n(x, 100) && !is_x_div_by_n(x, 400)
}

fn is_leap_year(yr: u32) -> bool {
    let mut result: bool = false;
    if is_div_by_4(yr) {
        result = true;
        if is_div_by_100_but_not_400(yr) {
            result = false;
        }
    }
    result
}

fn get_rand_num(min_incl: u32, max_incl: u32) -> u32 {
    rand::thread_rng().gen_range(min_incl..=max_incl)
}

fn get_rand_year() -> u32 {
    get_rand_num(1, 4001)
}

fn get_rand_month() -> u32 {
    get_rand_num(1, 12)
}

fn get_rand_day(days_in_month: u32) -> u32 {
    get_rand_num(1, days_in_month)
}

fn get_rand_date() -> String {
    let era: String = String::from("AD");
    let year: u32 = get_rand_year();
    let month: u32 = get_rand_month();
    let days_in_month: u32 = match month {
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    let day: u32 = get_rand_day(days_in_month);

    std::format!("y: {} {}, m: {}, d: {}", year, era, month, day)
}

fn main() {
    println!("Printing 10 random dates.\n");
    for _ in 1..10 {
        println!("{}", get_rand_date());
    }
    println!("\nThat's all. Goodbye!");
}

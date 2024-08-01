// literal translation of wikipedia's:
// "This extra leap day occurs in each year that is a multiple of 4, except for
// years evenly divisible by 100 but not by 400."
//
fn is_x_div_by_n(x: &u32, n: u32) -> bool {
    x % n == 0
}

fn is_div_by_4(x: &u32) -> bool {
    is_x_div_by_n(x, 4)
}

fn is_div_by_100_but_not_400(x: &u32) -> bool {
    is_x_div_by_n(x, 100) && !is_x_div_by_n(x, 400)
}

fn is_leap_year(yr: &u32) -> bool {
    let mut result: bool = false;
    if is_div_by_4(yr) {
        result = true;
        if is_div_by_100_but_not_400(yr) {
            result = false;
        }
    }
    result
}

fn main() {
    let yrs: Vec<u32> = vec![
        1700, 1800, 1900, 1904, 1914, 1950, 1980, 1985, 2000, 2004, 2024,
    ];
    println!("Testing possible leap years.\n");
    for yr in yrs.iter() {
        println!("Is {} a leap year? {}", yr, is_leap_year(yr))
    }
    println!("\nThat's all. Goodbye.");
}

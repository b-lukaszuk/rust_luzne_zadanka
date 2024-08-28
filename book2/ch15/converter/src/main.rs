// simple solution,
// not idiot/error-proof, not optimized for speed

fn celsius_to_fahrenheit(deg_cels: f64) -> Option<f64> {
    if deg_cels < -273.15 {
        return None;
    }
    return Some(deg_cels * (9.0 / 5.0) + 32.0);
}

fn fahrenheit_to_celsius(deg_fahr: f64) -> Option<f64> {
    if deg_fahr < -459.67 {
        return None;
    }
    return Some((deg_fahr - 32.0) * (5.0 / 9.0));
}

fn get_row(temp: f64, conv: fn(f64) -> Option<f64>) -> String {
    let temp2: Option<f64> = conv(temp);
    match temp2 {
        Some(t) => format!("| {:8.2} | {:8.2} |", temp, t),
        None => format!("| {:8} | {:8} |", "Err", "Err"),
    }
}

fn get_footer() -> String {
    String::from("=======================")
}

fn get_header(from_unit: String, to_unit: String) -> String {
    format!("| {:8} | {:8} |\n{}", from_unit, to_unit, get_footer())
}

fn print_table(conv: fn(f64) -> Option<f64>, from_unit: String, to_unit: String) {
    println!("{}", get_header(from_unit, to_unit));
    // by 20 to shorten the output
    for i in (-40..=100).step_by(20) {
        println!("{}", get_row(i as f64, conv));
    }
    println!("{}", get_footer());
}

fn main() {
    println!("Toy program. Temperature Converter Tables.");
    println!("No guarantee the output is correct.\n");

    print_table(
        celsius_to_fahrenheit,
        String::from("deg. C"),
        String::from("deg. F"),
    );
    println!();
    print_table(
        fahrenheit_to_celsius,
        String::from("deg. F"),
        String::from("deg. C"),
    );

    println!("\nThat's all. Goodbye!");
}

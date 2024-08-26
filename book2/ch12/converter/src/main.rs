// simple solution,
// not idiot/error-proof, not optimized for speed
use rand::Rng;

fn get_rand_int(min_incl: i32, max_incl: i32) -> i32 {
    rand::thread_rng().gen_range(min_incl..=max_incl)
}

fn celsius_to_fahrenheit(deg_cels: f64) -> Option<f64> {
    if deg_cels < -273.15 {
        return None;
    }
    return Some(deg_cels * (9.0 / 5.0) + 32.0);
}

fn kelvin_to_fahrenheit(deg_kelv: f64) -> Option<f64> {
    if deg_kelv < 0.0 {
        return None;
    }
    return Some(deg_kelv * (9.0 / 5.0) - 459.67);
}

fn print_conversion_examples(
    converter: fn(f64) -> Option<f64>,
    from_units: &str,
    to_units: &str,
    n_examples: u32,
) {
    for _ in 1..=n_examples {
        let temp = get_rand_int(-1000, 1000);
        let t_conv = converter(temp as f64);
        match t_conv {
            Some(t) => println!("{} {} is {:.2} {}.", temp, from_units, t, to_units),
            None => println!(
                "Cannot convert {} {} to {}. Conversion out of range.",
                temp, from_units, to_units
            ),
        }
    }
}

fn main() {
    println!("Toy program. Temperature Converter.");
    println!("No guarantee the results will be correct.\n");

    print_conversion_examples(celsius_to_fahrenheit, "deg. Celsius", "deg. Fahrenheit", 5);
    print_conversion_examples(kelvin_to_fahrenheit, "deg. Kelvin", "deg. Fahrenheit", 5);

    println!("\nThat's all. Goodbye!")
}

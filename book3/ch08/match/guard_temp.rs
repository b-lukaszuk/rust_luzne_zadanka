#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    // let temp = Temperature::Celsius(35);
    // let temp = Temperature::Celsius(0);
    // let temp = Temperature::Fahrenheit(90);
    let temp = Temperature::Fahrenheit(35);
    // ^ TODO try different values for `temperature`

    match temp {
        // `if condition` part is a guard
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius.", t),
        Temperature::Celsius(t) => println!("{}C is equal to or below 30 Celsius.", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit.", t),
        Temperature::Fahrenheit(t) => println!("{}F is equal to or below 86 Fahrenheit.", t),
    }
}

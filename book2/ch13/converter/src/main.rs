// simple solution,
// not idiot/error-proof, not optimized for speed
struct Celsius {
    degs: f64,
}

struct Kelvin {
    degs: f64,
}

struct Fahrenheit {
    degs: f64,
}

impl Celsius {
    fn to_fahrenheit(&self) -> Option<Fahrenheit> {
        if self.degs < -273.15 {
            return None;
        }
        return Some(Fahrenheit {
            degs: self.degs * (9.0 / 5.0) + 32.0,
        });
    }

    fn to_kelvin(&self) -> Option<Kelvin> {
        if self.degs < -273.15 {
            return None;
        }
        return Some(Kelvin {
            degs: self.degs + 273.15,
        });
    }
}

impl Kelvin {
    fn to_fahrenheit(&self) -> Option<Fahrenheit> {
        if self.degs < 0.0 {
            return None;
        }
        return Some(Fahrenheit {
            degs: self.degs * (9.0 / 5.0) - 459.67,
        });
    }

    fn to_celsius(&self) -> Option<Celsius> {
        if self.degs < 0.0 {
            return None;
        }
        return Some(Celsius {
            degs: self.degs - 273.15,
        });
    }
}

impl Fahrenheit {
    fn to_kelvin(&self) -> Option<Kelvin> {
        if self.degs < -459.67 {
            return None;
        }
        return Some(Kelvin {
            degs: (self.degs + 459.67) * (5.0 / 9.0),
        });
    }

    fn to_celsius(&self) -> Option<Celsius> {
        if self.degs < -459.67 {
            return None;
        }
        return Some(Celsius {
            degs: (self.degs - 32.0) * (5.0 / 9.0),
        });
    }
}

fn main() {
    println!("Toy program. Temperature Converter.");
    println!("No guarantee the results will be correct.\n");

    let c = Celsius { degs: 100.0 };
    let f = c.to_fahrenheit().unwrap();
    let k = c.to_kelvin().unwrap();
    println!("{:.2} deg. Cels. is {:.2} deg. Fahr.", c.degs, f.degs);
    println!("{:.2} deg. Cels. is {:.2} deg. Kelv.", c.degs, k.degs);

    let f = Fahrenheit { degs: 100.0 };
    let c = f.to_celsius().unwrap();
    let k = f.to_kelvin().unwrap();
    println!("{:.2} deg. Fahr. is {:.2} deg. Cels.", f.degs, c.degs);
    println!("{:.2} deg. Fahr. is {:.2} deg. Kelv.", f.degs, k.degs);

    let k = Kelvin { degs: 100.0 };
    let c = k.to_celsius().unwrap();
    let f = k.to_fahrenheit().unwrap();
    println!("{:.2} deg. Kelv. is {:.2} deg. Cels.", k.degs, c.degs);
    println!("{:.2} deg. Kelv. is {:.2} deg. Fahr.", k.degs, f.degs);

    println!("\nThat's all. Goodbye!")
}

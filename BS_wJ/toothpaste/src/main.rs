use std::f64::consts::PI;

#[derive(Debug)]
struct Cylinder {
    radius: u32,
    height: u32,
}

fn get_volume(c: &Cylinder) -> f64 {
    (c.height as f64) * PI * (c.radius.pow(2) as f64)
}
fn get_ratios(cylinders: &Vec<Cylinder>, radius_change: i32, height_change: i32) -> Vec<f64> {
    let mut ratios: Vec<f64> = vec![];
    for cyl1 in cylinders {
        let cyl2 = Cylinder {
            radius: ((cyl1.radius as i32) + radius_change) as u32,
            height: ((cyl1.height as i32) + height_change) as u32,
        };
        ratios.push(get_volume(&cyl2) / get_volume(&cyl1));
    }
    ratios
}

fn print_intro() {
    println!("Toy program for the toothpaste problem.");
    println!("Warning. It may or may not work correctly.\n");
}

fn print_outro() {
    println!("\nThat's all. Goodbye!");
}

fn print_problem_description() {
    println!("What happen with the consumption of toothpaste if:");
    println!("\tScenario 1. You increase radius by 1, and leave height as it is.");
    println!("\tScenario 2. You increase radius by 1, and decrease height by 1.");
    println!("\tScenario 3. You increase radius by 1, and decrease height by 2.");
}

fn print_scenario_info(
    scenario_number: u32,
    cylinders: &Vec<Cylinder>,
    radius_change: i32,
    height_change: i32,
) {
    println!(
        "\nScenario: {}, radius change: {}, height change: {}",
        scenario_number, radius_change, height_change,
    );
    println!("Applying change, consumption rate changed to:");
    let ratios: Vec<f64> = get_ratios(cylinders, radius_change, height_change);
    for rat in ratios {
        print!("{:.2}% ", rat * 100.0);
    }
    println!("");
}

fn print_solution() {
    // reference cylinders
    let mut cs: Vec<Cylinder> = vec![];
    for r in 1..=5 {
        cs.push(Cylinder {
            radius: r,
            height: 5,
        })
    }
    println!("\n\nInitial cylinders {:?}\n", cs);
    print_scenario_info(1, &cs, 1, 0);
    print_scenario_info(2, &cs, 1, -1);
    print_scenario_info(3, &cs, 1, -2);
}

fn main() {
    print_intro();

    print_problem_description();
    print_solution();

    print_outro();
}

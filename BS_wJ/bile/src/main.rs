use std::f64::consts::PI;

struct Sphere {
    radius: f64,
}

fn get_volume(s: &Sphere) -> f64 {
    4.0 / 3.0 * PI * (s.radius.powf(3.0) as f64)
}

fn get_surface_area(s: &Sphere) -> f64 {
    4.0 * PI * (s.radius.powf(2.0) as f64)
}

fn get_sphere(volume: f64) -> Sphere {
    Sphere {
        radius: (volume / (4.0 / 3.0) / PI).cbrt(),
    }
}

fn print_intro() {
    println!("Toy program for the bile problem.");
    println!("Warning. It may or may not work correctly.\n");
}

fn print_outro() {
    println!("\nThat's all. Goodbye!");
}

fn print_solution() {
    let initial_sphere: Sphere = Sphere { radius: 10.0 };
    let initial_volume: f64 = get_volume(&initial_sphere);
    let mut spheres: Vec<Sphere> = vec![initial_sphere];
    let mut num_spheres: Vec<u32> = vec![1];
    let mut sum_of_volumes: Vec<f64> = vec![get_volume(&spheres[0])];
    let mut sum_of_areas: Vec<f64> = vec![get_surface_area(&spheres[0])];

    for n_drops in (4..=12).step_by(4) {
        num_spheres.push(n_drops);
        spheres.push(get_sphere(initial_volume / n_drops as f64));
        sum_of_volumes.push(get_volume(&spheres.last().unwrap()) * n_drops as f64);
        sum_of_areas.push(get_surface_area(&spheres.last().unwrap()) * n_drops as f64);
    }
    for i in 0..spheres.len() {
        println!(
        "{} droplet(s):\n\tr (1 droplet) = {:.2} [um], sumArea = {:.2} [um^2], sumVolume = {:.2} [um^3]",
        num_spheres[i], spheres[i].radius, sum_of_areas[i], sum_of_volumes[i]
    );
    }
}

fn main() {
    print_intro();

    print_solution();

    print_outro();
}

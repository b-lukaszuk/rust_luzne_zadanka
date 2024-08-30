// simple solution,
// not idiot/error-proof, not optimized for speed

type Planets = Vec<String>;

fn is_x_within_range(x: usize, min_incl: usize, max_incl: usize) -> bool {
    (x >= min_incl) && (x <= max_incl)
}

fn terraform(planets: &mut Planets, inds: Vec<usize>) {
    for i in inds {
        if is_x_within_range(i, 0, planets.len()) {
            planets[i] = format!("New {}", planets[i]);
        }
    }
}

fn main() {
    println!("Toy program.");
    println!("It 'terraforms' planets from a vector.\n");

    let mut planets: Planets = vec![
        "Mercury", "Venus", "Earth", "Mars", "Jupiter", "Saturn", "Uranus", "Neptune",
    ]
    .iter()
    .map(|txt| txt.to_string())
    .collect();
    println!("Planets before: {:?}", planets);
    terraform(&mut planets, vec![3, 6, 7]);
    println!("Terraforming some planets, result: {:?}", planets);

    println!("\nThat's all. Goodbye!");
}

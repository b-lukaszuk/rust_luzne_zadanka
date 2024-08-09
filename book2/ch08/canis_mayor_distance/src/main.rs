fn main() {
    const CANIS_MAJOR_DISTANCE_KM: f64 = 236e15;
    const LIGHT_SPEED_KM_PER_SEC: f64 = 299792.0;
    const SECS_IN_EARTH_YR: f64 = 60.0 * 60.0 * 24.0 * 365.2422;
    let canis_major_distance_light_yrs: f64 =
        CANIS_MAJOR_DISTANCE_KM / LIGHT_SPEED_KM_PER_SEC / SECS_IN_EARTH_YR;

    println!("Toy program.");
    println!("It estimates the distance from the Sun to Canis Major galaxy.\n");
    println!(
        "The galaxy Canis Major is ~{:.2} [km] away from the Sun.",
        CANIS_MAJOR_DISTANCE_KM
    );
    println!(
        "To travel this distance light needs ~{:.2} [years].",
        canis_major_distance_light_yrs
    );

    println!("\nThat's all. Goodbye!");
}

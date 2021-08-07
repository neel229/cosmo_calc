use std::io;

// surface gravities of different planets
const SG_MERCURY: f32 = 0.38;
const SG_VENUS: f32 = 0.91;
const SG_MARS: f32 = 0.38;
const SG_JUPITER: f32 = 2.34;
const SG_SATURN: f32 = 0.93;
const SG_URANUS: f32 = 0.92;
const SG_NEPTUNE: f32 = 1.12;

fn main() {
    println!("Enter your weight (in kgs): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    println!("Enter the name of planet");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let planet = input1.trim().to_lowercase();
    let cosmos_weight = calc_weight(weight, planet.as_str()).unwrap();
    println!("Your weight on planet {} is : {}", planet, cosmos_weight);
}

fn calc_weight(weight: f32, planet: &str) -> Result<f32, &str> {
    match planet {
        "mercury" => return Ok(weight * SG_MERCURY),
        "venus" => return Ok(weight * SG_VENUS),
        "mars" => return Ok(weight * SG_MARS),
        "jupiter" => return Ok(weight * SG_JUPITER),
        "saturn" => return Ok(weight * SG_SATURN),
        "neptune" => return Ok(weight * SG_NEPTUNE),
        "uranus" => return Ok(weight * SG_URANUS),
        "pluto" => return Err("Pluto is no more a planet"),
        _ => return Err("Please provide a planet name"),
    }
}

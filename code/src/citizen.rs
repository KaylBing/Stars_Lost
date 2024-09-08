// This file will be the main class used for all citizens in the game //

use rand::Rng; // For generating random numbers //
use rand::seq::SliceRandom; // For selecting random elements from a list //

// Define the Citizen struct with public fields //
// Will have variations for each government type //

pub struct Citizen {
    pub name: String,
    pub age: u32,
    // Mood to be expanded into multiple values //
    pub mood: u8,
    // Weapon & fighting skills //
    pub unarmed: u8,
    pub blades: u8,
    pub handguns: u8,
    pub rifles: u8,
    pub turrets: u8,
    // Engineering Skills //
    pub electrical: u8,
    pub structural: u8,
    pub propulsion: u8,
    pub hydro: u8, 
    // Science Skills //
    pub biology: u8,
    pub math: u8,
    pub chemistry: u8,
    pub astronomy: u8,
    pub physics: u8,
    // Other Skills //
    pub literature: u8, 
    pub poetry: u8,
    pub history: u8,
    pub philosophy: u8,
    // Conditions //
    pub is_alive: bool,
    // Government //
    pub government: String,
    pub empathy: u8,  // Changed from humanity to empathy
    pub greedy: bool,
}

// Function to create a new Citizen with government-specific values //
// To be used as the base for all creation functions once complete //
pub fn create_citizen(names: &[&str], government: &str) -> Citizen {
    let mut rng = rand::thread_rng();
    
    let (min_emp, max_emp) = if government == "corporate" { // If corporate start with a lower average empathy //
        (0, 8)
    } else {
        (0, 10) // Normal range //
    };

    let greed_chance = if government == "corporate" {  // 50% chance for corporate colonists to be greedy //
        0.5
    } else {
        0.05 // 5% chance otherwise //
    };

    let greedy = rng.gen_bool(greed_chance);

    Citizen {
        name: names.choose(&mut rng).unwrap_or(&"Unknown").to_string(),
        age: rng.gen_range(18..=80),
        mood: rng.gen_range(0..=10),
        unarmed: rng.gen_range(0..=10),
        blades: rng.gen_range(0..=10),
        handguns: rng.gen_range(0..=10),
        rifles: rng.gen_range(0..=10),
        turrets: rng.gen_range(0..=10),
        electrical: rng.gen_range(0..=14),
        structural: rng.gen_range(0..=14),
        propulsion: rng.gen_range(0..=14),
        hydro: rng.gen_range(0..=14),
        biology: rng.gen_range(0..=10),
        math: rng.gen_range(0..=10),
        chemistry: rng.gen_range(0..=10),
        astronomy: rng.gen_range(0..=10),
        physics: rng.gen_range(0..=10),
        literature: rng.gen_range(0..=10),
        poetry: rng.gen_range(0..=10),
        history: rng.gen_range(0..=10),
        philosophy: rng.gen_range(0..=10),
        empathy: rng.gen_range(min_emp..=max_emp),
        greedy,
        is_alive: true,
        government: government.to_string(),
    }
}

pub fn display_citizen(citizen: &Citizen) {
    // Display the citizen's attributes //
    println!("Name: {}", citizen.name);
    println!("Age: {}", citizen.age);
    println!("Mood: {}", citizen.mood);
    println!("Unarmed: {}", citizen.unarmed);
    println!("Blades: {}", citizen.blades);
    println!("Handguns: {}", citizen.handguns);
    println!("Rifles: {}", citizen.rifles);
    println!("Turrets: {}", citizen.turrets);
    println!("Electrical: {}", citizen.electrical);
    println!("Structural: {}", citizen.structural);
    println!("Propulsion: {}", citizen.propulsion);
    println!("Hydro: {}", citizen.hydro);
    println!("Biology: {}", citizen.biology);
    println!("Math: {}", citizen.math);
    println!("Chemistry: {}", citizen.chemistry);
    println!("Astronomy: {}", citizen.astronomy);
    println!("Physics: {}", citizen.physics);
    println!("Literature: {}", citizen.literature);
    println!("Poetry: {}", citizen.poetry);
    println!("History: {}", citizen.history);
    println!("Philosophy: {}", citizen.philosophy);
    println!("Empathy: {}", citizen.empathy);
    println!("Greedy: {}", citizen.greedy);
    println!("Is Alive: {}", citizen.is_alive);
}

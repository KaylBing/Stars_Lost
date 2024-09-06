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
}

// Function to create a new Citizen with higher than normal engineering values //
pub fn create_soviet_citizen(names: &[&str]) -> Citizen {
    let mut rng = rand::thread_rng();
    
    Citizen {
        name: names.choose(&mut rng).unwrap_or(&"Unknown").to_string(),
        age: rng.gen_range(18..=124),
        mood: rng.gen_range(0..=10),
        unarmed: rng.gen_range(0..=10),
        blades: rng.gen_range(0..=10),
        handguns: rng.gen_range(0..=10),
        rifles: rng.gen_range(0..=10),
        turrets: rng.gen_range(0..=10),
        electrical: rng.gen_range(2..=12),
        structural: rng.gen_range(2..=12),
        propulsion: rng.gen_range(2..=12),
        hydro: rng.gen_range(2..=12),
        biology: rng.gen_range(0..=10),
        math: rng.gen_range(0..=10),
        chemistry: rng.gen_range(0..=10),
        astronomy: rng.gen_range(0..=10),
        physics: rng.gen_range(0..=10),
        literature: rng.gen_range(0..=10),
        poetry: rng.gen_range(0..=10),
        history: rng.gen_range(0..=10),
        philosophy: rng.gen_range(0..=10),
        is_alive: true,
    }
}

// This file will be the main class used for all citizens in the game //
// Sorry in advance for my technical debt //

use rand::Rng; // For generating random numbers //
use rand::seq::SliceRandom; // For selecting random elements from a list //

// Define the Citizen struct //
struct Citizen {
    name: String,
    age: u32,
    mood: u8,
    fightiness: u8,
    engineering: u8,
    science: u8, 
    humanity: u8, // TODO rethink these if needed //
}

// Function to create a new Citizen with random values //
fn create_random_citizen(names: &[&str]) -> Citizen {
    let mut rng = rand::thread_rng();
    
    Citizen {
        name: names.choose(&mut rng).unwrap_or(&"Unknown").to_string(),
        age: rng.gen_range(18..=100),
        mood: rng.gen_range(6..=10),
        fightiness: rng.gen_range(0..=10),
        engineering: rng.gen_range(0..=10),
        science: rng.gen_range(0..=10),
        humanity: rng.gen_range(0..=10),
    }
}

// Test, to be deleted later //
fn main() {
    let names = ["Cy", "Cargo", "Evo", "Olga"];

    // Create a random citizen //
    let random_citizen = create_random_citizen(&names);

    // Display the random citizen //
    println!("Name: {}", random_citizen.name);
    println!("Age: {}", random_citizen.age);
    println!("Mood: {}", random_citizen.mood);
    println!("Fightiness: {}", random_citizen.fightiness);
    println!("Engineering: {}", random_citizen.engineering);
    println!("Science: {}", random_citizen.science);
    println!("Humanity: {}", random_citizen.humanity);
}

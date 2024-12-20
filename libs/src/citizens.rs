// This file holds the functions used for all citizens in the game //
// The goal is to keep each citizen under one megabyte of ram in size //

use rand::Rng; // For generating random numbers //
use rand::seq::SliceRandom; // For selecting random elements from a list //
use std::mem; // For calculating memory size //
use std::io::{self, Write}; // For speaking with citizens //
use std::collections::HashMap; // For storing learned data //

pub struct Citizen { 
    pub name: String,
    pub age: u16,
    pub occupation: String, // Based on the sum of each skill family // 
    // TODO Add ethnicity string //
    // Mood and personality values //
    pub mood: String, // Will be linked to other factors eventually //
    pub empathy: u8, // More likely to share and be affected by others moods // 
    pub greedy: bool, // Greedy characters are more likely to steal or cheat //
    pub knowledge: HashMap<String, String>,  // Stores learned questions/answers //
    // Weapon values //
    pub unarmed: u8,
    pub blades: u8,
    pub handguns: u8,
    pub rifles: u8,
    pub turrets: u8,
    // Engineering values //
    pub electrical: u8,
    pub structural: u8,
    pub propulsion: u8,
    pub hydro: u8, 
    // Science values //
    pub biology: u8,
    pub math: u8,
    pub chemistry: u8,
    pub astronomy: u8,
    pub physics: u8,
    // Humanities values //
    pub literature: u8, 
    pub poetry: u8,
    pub history: u8,
    pub philosophy: u8,
    // Misc values //
    pub is_alive: bool,
    pub government: String,
}

pub fn create_citizen(names: &[&str], government: &str) -> Citizen {
    let mut rng = rand::thread_rng(); 
    
    let (min_emp, max_emp) = if government == "corporate" { // If corporate start with a lower average empathy //
        (0, 8)
    } else {
        (4, 10) // Normal range //
    };

    let greed_chance = if government == "corporate" {  // 50% chance for corporate colonists to be greedy //
        0.5
    } else {
        0.05 // 5% chance otherwise //
    };

    let greedy = rng.gen_bool(greed_chance); 

    // Generate the values for the citizen attributes
    let unarmed = rng.gen_range(0..=10);
    let blades = rng.gen_range(0..=10);
    let handguns = rng.gen_range(0..=10);
    let rifles = rng.gen_range(0..=10);
    let turrets = rng.gen_range(0..=10);
    let electrical = rng.gen_range(0..=10);
    let structural = rng.gen_range(0..=10);
    let propulsion = rng.gen_range(0..=10);
    let hydro = rng.gen_range(0..=10);
    let biology = rng.gen_range(0..=10);
    let math = rng.gen_range(0..=10);
    let chemistry = rng.gen_range(0..=10);
    let astronomy = rng.gen_range(0..=10);
    let physics = rng.gen_range(0..=10);
    let literature = rng.gen_range(0..=10);
    let poetry = rng.gen_range(0..=10);
    let history = rng.gen_range(0..=10);
    let philosophy = rng.gen_range(0..=10);

    // Group the values and find the maximum sum
    let group_sums = [
        unarmed + blades + handguns,
        rifles + turrets + electrical,
        structural + propulsion + hydro,
        biology + math + chemistry,
        astronomy + physics + literature,
        poetry + history + philosophy,
    ];

    // Calculate the maximum sum and determine occupation
    let mut max_sum = group_sums[0];
    let mut occupation = "Unknown"; // Default occupation
    for (i, &sum) in group_sums.iter().enumerate() {
        if sum > max_sum {
            max_sum = sum;
            occupation = match i {
                0 => "Combat Specialist",
                1 => "Engineer",
                2 => "Mechanic",
                3 => "Scientist",
                4 => "Astronomer",
                5 => "Historian",
                _ => "Citizen",
            };
        }
    }

    // Return a Citizen object
    Citizen {
        name: names.choose(&mut rng).unwrap_or(&"Unknown").to_string(),
        age: rng.gen_range(18..=60), // Random age
        occupation: occupation.to_string(),
        mood: "Neutral".to_string(), // Default mood
        empathy: rng.gen_range(min_emp..=max_emp),
        greedy,
        knowledge: HashMap::new(),
        unarmed,
        blades,
        handguns,
        rifles,
        turrets,
        electrical,
        structural,
        propulsion,
        hydro,
        biology,
        math,
        chemistry,
        astronomy,
        physics,
        literature,
        poetry,
        history,
        philosophy,
        is_alive: true,
        government: government.to_string(),
    }
}

pub fn display_citizen(citizen: &Citizen) {
    println!("Name: {}", citizen.name); 
    println!("Age: {}", citizen.age); 
    println!("Mood: {}", citizen.mood);
    println!("Occupation: {}", citizen.occupation);
    println!("Empathy: {}", citizen.empathy);
    println!("Greedy: {}", citizen.greedy); 
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
    println!("Is Alive: {}", citizen.is_alive);
    println!("Government: {}", citizen.government); 

    let size = mem::size_of_val(citizen); 
    println!("Memory usage of this citizen: {} bytes", size); 
}

// A simple tokenizer function that splits input into words //
fn tokenize(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

// Function for the citizen to speak, with learning capability //
pub fn speak_with_citizen(citizen: &mut Citizen) {
    loop {
        // Prompt user for input
        print!("You: ");
        io::stdout().flush().unwrap();  // Ensure the prompt is shown immediately //

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let user_input = user_input.trim().to_string(); // Remove any extra whitespace/newlines //

        // Exit the loop if user types "exit" //
        if user_input.eq_ignore_ascii_case("exit") {
            println!("Citizen {} says: Goodbye!", citizen.name);
            break;
        }

        // Check if the user input is a known question
        if let Some(response) = citizen.knowledge.get(&user_input) {
            println!("Citizen {} says: {}", citizen.name, response);
        } else {
            // If the question is unknown, ask the user to teach the citizen (temp for testing) //
            println!("Citizen {} doesn't know how to respond to that. What should the answer be?", citizen.name);
            let mut new_response = String::new();
            io::stdin().read_line(&mut new_response).expect("Failed to read line");
            let new_response = new_response.trim().to_string();

            // Save the new question/answer pair to the citizen's knowledge //
            citizen.knowledge.insert(user_input, new_response);
            println!("Citizen {} has learned a new response!", citizen.name);
        }
    }
}

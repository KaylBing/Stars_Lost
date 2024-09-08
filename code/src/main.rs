mod citizen; // Import citizen //

fn main() {
    // Define modern Canadian names
    let names = [
        "Liam", "Emma", "Noah", "Olivia", "Ethan", "Ava",
        "Lucas", "Sophia", "Mason", "Isabella"
    ];

    // Create a random citizen with 'corporate' government
    let test_citizen = citizen::create_citizen(&names, "corporate");

    // Display the random citizen
    println!("Name: {}", test_citizen.name);
    println!("Age: {}", test_citizen.age);
    println!("Mood: {}", test_citizen.mood);
    println!("Unarmed: {}", test_citizen.unarmed);
    println!("Blades: {}", test_citizen.blades);
    println!("Handguns: {}", test_citizen.handguns);
    println!("Rifles: {}", test_citizen.rifles);
    println!("Turrets: {}", test_citizen.turrets);
    println!("Electrical: {}", test_citizen.electrical);
    println!("Structural: {}", test_citizen.structural);
    println!("Propulsion: {}", test_citizen.propulsion);
    println!("Hydro: {}", test_citizen.hydro);
    println!("Biology: {}", test_citizen.biology);
    println!("Math: {}", test_citizen.math);
    println!("Chemistry: {}", test_citizen.chemistry);
    println!("Astronomy: {}", test_citizen.astronomy);
    println!("Physics: {}", test_citizen.physics);
    println!("Literature: {}", test_citizen.literature);
    println!("Poetry: {}", test_citizen.poetry);
    println!("History: {}", test_citizen.history);
    println!("Philosophy: {}", test_citizen.philosophy);
    println!("Empathy: {}", test_citizen.empathy);
    println!("Is Alive: {}", test_citizen.is_alive);
}
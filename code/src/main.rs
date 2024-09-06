mod citizen; // Import citizen //

// Test, in active development //
fn main() {
    let names = ["Cy", "Cargo", "Evo", "Olga", "Mikhail", "George", "Odessa", "Jaroslav", "Zoya", "Halya"];

    // Create a random citizen //
    let test_citizen = citizen::create_soviet_citizen(&names);  // Call the function from the `citizen` module

    // Display the random citizen //
    println!("Name: {}", test_citizen.name);
    println!("Age: {}", test_citizen.age);
    println!("Mood: {}", test_citizen.mood);
}

// Main file for the entire game //
// Defines relationships between different functions etc //
mod citizens;

fn main() {
    // List of names to choose from //
    let names = ["Yashkeno", "Mikhail", "Yaroslav", "Sophia", "Vladmir"];

    // Create a citizen with the chosen government type //
    let new_citizen = citizens::create_citizen(&names, "corporate");

    // Display the created citizen //
    citizens::display_citizen(&new_citizen);

    // Start conversation with the citizen //
    citizens::speak_with_citizen(&new_citizen);
}

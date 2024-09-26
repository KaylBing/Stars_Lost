// Main file for the entire game //
// Defines relationships between different functions etc //
mod citizens;

use rand::seq::SliceRandom; // For selecting random elements from a list //
use citizens::create_citizen; // Ensure you're importing the create_citizen function

fn main() {
    // List of names to choose from //
    let names = ["Yashkeno", "Mikhail", "Yaroslav", "Sophia", "Vladmir"];

    // Create a mutable citizen with the chosen government type //
    let mut new_citizen = create_citizen(&names, "corporate");

    // Display the created citizen //
    citizens::display_citizen(&new_citizen);

    // Start a conversation with the citizen
    citizens::speak_with_citizen(&mut new_citizen); // Pass mutable reference
}

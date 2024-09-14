// Main file for the entire game //
mod citizen;

fn main() {
    // List of names to choose from //
    let names = ["Alice", "Bob", "Charlie", "Diana", "Edward"];

    // Create a citizen with the chosen government type //
    let new_citizen = citizen::create_citizen(&names, "corporate");

    // Display the created citizen //
    citizen::display_citizen(&new_citizen);
}

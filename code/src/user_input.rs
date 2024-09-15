// This files functions handle user input //

// Define the trait that acts like a command interface //
pub trait Command {
    fn execute(&self);  // The method that all commands will implement //
    fn undo(&self); // Some commands will also have undo statements, that will only work if the game is paused //
}

// Command that will monitor for "w", which will move the users screen up //
pub struct move_up;

impl Command for move_up {
    fn execute(&self) {
        println!("Executing move up...");
        // In reality this would call the move up function that moves the screen //
    }
}
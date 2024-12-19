// Main file for the entire game //
// Defines relationships between different functions etc //

use ggez::{event, Context, ContextBuilder, GameResult};
use std::path::PathBuf;

mod display;
#[path = "citizens.rs"]
mod citizen;


fn main() -> GameResult {
    let (mut ctx, mut event_loop) = ContextBuilder::new("tile_system_game", "Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(
            display::WINDOW_WIDTH,
            display::WINDOW_HEIGHT,
        ))
        .add_resource_path(PathBuf::from("./resources"))
        .build()?;

    let state = display::GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

// This holds all of the functions that display output to the user //
// They mostly are modifications to the array that holds the main //
// pixels on the screen //

use ggez::{event::EventHandler, graphics, Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawParam, Text};

// Display window variables //
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

// Tile (background) struct //
pub struct Tile {
    ascii_char: char,
    is_walkable: bool,
}

// Struct for characters and monsters //
pub struct Entity {
    ascii_char: char,
    x: f32,
    y: f32,
}

// Imports models and implements them //
pub struct GameState;

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameResult<GameState> {
        Ok(GameState)
    }

    fn draw_tile(
        &self,
        canvas: &mut Canvas,
        tile: &Tile,
        x: usize,
        y: usize,
    ) -> GameResult {
        let position = [x as f32 * 16.0, y as f32 * 16.0]; // Adjust to fit text size
        let text = Text::new(tile.ascii_char.to_string());

        canvas.draw(&text, DrawParam::default().dest(position));
        Ok(())
    }

    fn draw_entity(
        &self,
        canvas: &mut Canvas,
        entity: &Entity,
    ) -> GameResult {
        let position = [entity.x, entity.y];
        let text = Text::new(entity.ascii_char.to_string());

        canvas.draw(&text, DrawParam::default().dest(position));
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update game state here
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        // Example: Draw a tile and an entity
        let tile = Tile { ascii_char: '.', is_walkable: true };
        let entity = Entity { ascii_char: '@', x: 100.0, y: 100.0 };

        self.draw_tile(&mut canvas, &tile, 0, 0)?;
        self.draw_entity(&mut canvas, &entity)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}

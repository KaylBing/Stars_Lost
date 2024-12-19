// This holds all of the functions that display output to the user //
// They mostly are modifications to the array that holds the main //
// pixels on the screen //

use ggez::{event::EventHandler, graphics, Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawParam, Text};
use crate::citizen::{create_citizen, Citizen};

// Display window variables //
pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

// Tile (background and walls) struct //
pub struct Tile {
    ascii_char: char,
    color: Color,
    is_walkable: bool,
}

// Struct for characters (e.g., citizens) //
pub struct Entity {
    ascii_char: char,
    color: Color,
    x: f32,
    y: f32,
    citizen: Citizen, // Link to a Citizen instance
}

// Imports models and implements them //
pub struct GameState {
    tiles: Vec<Tile>,      // Example: A grid of tiles for walls
    entities: Vec<Entity>, // A list of entities, including citizens
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameResult<GameState> {
        // Create a random citizen
        let names = ["Alice", "Bob", "Charlie", "Diana"];
        let government = "Capitalist Democracy"; // Example government type
        let citizen = create_citizen(&names, government);

        // Create an entity to represent the citizen
        let citizen_entity = Entity {
            ascii_char: '@',
            color: match citizen.mood.as_str() {
                "Happy" => Color::GREEN,
                "Angry" => Color::RED,
                "Neutral" => Color::WHITE,
                _ => Color::WHITE,
            },
            x: 100.0,
            y: 100.0,
            citizen,
        };

        // Example wall tiles
        let tiles = vec![
            Tile {
                ascii_char: '-',
                color: Color::WHITE,
                is_walkable: false,
            },
            Tile {
                ascii_char: '|',
                color: Color::WHITE,
                is_walkable: false,
            },
            Tile {
                ascii_char: ':',
                color: Color::WHITE,
                is_walkable: false,
            },
        ];

        Ok(GameState {
            tiles,
            entities: vec![citizen_entity], // Add the citizen as the only entity for now
        })
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
        canvas.draw(
            &text,
            DrawParam::default()
                .dest(position)
                .color(tile.color), // Apply the tile's color
        );
        Ok(())
    }

    fn draw_entity(
        &self,
        canvas: &mut Canvas,
        entity: &Entity,
    ) -> GameResult {
        let position = [entity.x, entity.y];
        let text = Text::new(entity.ascii_char.to_string());
        canvas.draw(
            &text,
            DrawParam::default()
                .dest(position)
                .color(entity.color), // Apply the entity's color
        );
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

        // Draw all tiles (example positions)
        for (i, tile) in self.tiles.iter().enumerate() {
            self.draw_tile(&mut canvas, tile, i, 0)?; // Example row of tiles
        }

        // Draw all entities
        for entity in &self.entities {
            self.draw_entity(&mut canvas, entity)?;
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

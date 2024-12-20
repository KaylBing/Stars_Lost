// This holds all of the functions that display output to the user //
// They mostly are modifications to the array that holds the main //
// pixels on the screen //

use ggez::{event::EventHandler, graphics, Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawParam, Text};
use crate::citizen::{create_citizen, Citizen};
use sysinfo::{System, SystemExt}; // Added sysinfo for system stats

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
    frame_counter: u64,    // Frame counter to track how many frames have been drawn
    system: System,        // System object to track memory usage and stats
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

        // Initialize system for stats (memory usage)
        let system = System::new_all();

        Ok(GameState {
            tiles,
            entities: vec![citizen_entity], // Add the citizen as the only entity for now
            frame_counter: 0,  // Initialize frame counter
            system,            // Store system info object
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

    fn draw_stats(
        &mut self, // Mutable reference to allow refreshing memory
        canvas: &mut Canvas,
        ctx: &mut Context,
    ) -> GameResult {
        // Get FPS
        let fps = ctx.time.fps();
    
        // Refresh memory usage
        self.system.refresh_memory();
        let memory = self.system.used_memory() as f64 / (1024.0 * 1024.0); // Memory in MB
    
        // Prepare text to display the frame count, FPS, and memory usage
        let stats_text = format!(
            "Frames: {}\nFPS: {:.2}\nMemory: {:.2} MB",
            self.frame_counter, fps, memory
        );
        let text = Text::new(stats_text);
        let position = [WINDOW_WIDTH - 200.0, 10.0]; // Position at top right
    
        canvas.draw(
            &text,
            DrawParam::default()
                .dest(position)
                .color(Color::WHITE),
        );
    
        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Increment frame counter
        self.frame_counter += 1;

        // Update game state here (refresh system memory usage)
        self.system.refresh_all();

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

        // Draw stats (frame counter, FPS, memory usage)
        self.draw_stats(&mut canvas, ctx)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}

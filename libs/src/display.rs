// This holds all of the functions that display output to the user //
// They mostly are modifications to the array that holds the main //
// pixels on the screen //

use ggez::{event::EventHandler, graphics, Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawParam, Image, Rect};

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub struct Tile {
    sprite_index: usize,
    is_walkable: bool,
}

pub struct Entity {
    sprite_index: usize,
    x: f32,
    y: f32,
}

pub struct GameState {
    sprite_sheet: Image,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        let sprite_sheet = Image::from_path(ctx, "/sprite_sheet.png")?;
        Ok(GameState { sprite_sheet })
    }

    fn draw_tile(
        &self,
        canvas: &mut Canvas,
        tile: &Tile,
        x: usize,
        y: usize,
    ) -> GameResult {
        let sprite_width = self.sprite_sheet.width() as f32 / 8.0;
        let sprite_height = self.sprite_sheet.height() as f32 / 8.0;

        let sprite_rect = Rect::new(
            (tile.sprite_index as f32 % 8.0) * sprite_width,
            (tile.sprite_index as f32 / 8.0).floor() * sprite_height,
            sprite_width,
            sprite_height,
        );

        canvas.draw(
            &self.sprite_sheet,
            DrawParam::default()
                .src(sprite_rect)
                .dest([x as f32 * sprite_width, y as f32 * sprite_height]),
        );
        Ok(())
    }

    fn draw_entity(
        &self,
        canvas: &mut Canvas,
        entity: &Entity,
    ) -> GameResult {
        let sprite_width = self.sprite_sheet.width() as f32 / 8.0;
        let sprite_height = self.sprite_sheet.height() as f32 / 8.0;

        let sprite_rect = Rect::new(
            (entity.sprite_index as f32 % 8.0) * sprite_width,
            (entity.sprite_index as f32 / 8.0).floor() * sprite_height,
            sprite_width,
            sprite_height,
        );

        canvas.draw(
            &self.sprite_sheet,
            DrawParam::default()
                .src(sprite_rect)
                .dest([entity.x, entity.y]),
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
        
        // Example: Draw a tile and an entity
        let tile = Tile { sprite_index: 0, is_walkable: true };
        let entity = Entity { sprite_index: 1, x: 100.0, y: 100.0 };

        self.draw_tile(&mut canvas, &tile, 0, 0)?;
        self.draw_entity(&mut canvas, &entity)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}

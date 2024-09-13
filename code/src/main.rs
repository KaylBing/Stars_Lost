extern crate sdl2;

use imgref::Img;
use rgb::RGBA8;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create a window
    let window = video_subsystem
        .window("Stars Lost", 1000, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Get the window surface (where we can draw)
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    // Define screen dimensions
    let width = 1000;
    let height = 800;

    // Create an image buffer filled with gray color (128, 128, 128)
    let gray_color = RGBA8::new(128, 128, 128, 255); // Gray with full opacity
    let image_buffer = Img::new(vec![gray_color; width * height], width, height);

    // Convert the RGBA8 buffer to a Vec<u8> (raw bytes)
    let mut raw_buffer: Vec<u8> = Vec::with_capacity(width * height * 4); // 4 bytes per pixel (RGBA)
    for pixel in image_buffer.buf() {
        raw_buffer.push(pixel.r);
        raw_buffer.push(pixel.g);
        raw_buffer.push(pixel.b);
        raw_buffer.push(pixel.a);
    }

    // Create an SDL2 surface and populate it with the raw byte buffer
    let surface = Surface::from_data(
        &mut raw_buffer,   // The buffer itself
        width as u32,
        height as u32,
        width as u32 * 4,  // pitch: the length of a row in bytes (width * 4 for RGBA)
        PixelFormatEnum::RGBA32,
    )?;

    // Blit (copy) the surface onto the canvas and present it
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    canvas.copy(&texture, None, Rect::new(0, 0, width as u32, height as u32))?;
    canvas.present();

    // Wait for the window to remain open for a few seconds or until quit event
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }

    Ok(())
}

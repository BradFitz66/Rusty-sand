#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//Modules
mod lib;

//Imports
use beryllium::*;
use pixels::{Pixels, SurfaceTexture};
use lib::Universe;

//Constants
const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const PIXEL_SIZE:u32 = 4;
const WINDOW_WIDTH: u32 = WIDTH * PIXEL_SIZE;
const WINDOW_HEIGHT: u32 = HEIGHT * PIXEL_SIZE;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct FallingSandState {
    universe: Universe,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl = SDL::init(InitFlags::default())?;
    let window =
        sdl.create_raw_window("Hello Pixels", WindowPosition::Centered, WINDOW_WIDTH, WINDOW_HEIGHT, 0)?;

    let mut pixels = {
        // TODO: Beryllium does not expose the SDL2 `GetDrawableSize` APIs, so choosing the correct
        // surface texture size is not possible.
        let surface_texture = SurfaceTexture::new(WINDOW_WIDTH, WINDOW_HEIGHT, &window);
        Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture)?
    };
    let mut falling_sand_state= FallingSandState::new();
    for x in 0..WIDTH-1{
        for y in 0..15 {
            falling_sand_state.universe.set_cell_at(x as usize, y, lib::ParticleTypes::Sand)
        }
    }
    'game_loop: loop {
        match sdl.poll_events().and_then(Result::ok) {
            // Close events
            Some(Event::Quit { .. }) => break 'game_loop,
            Some(Event::Keyboard(KeyboardEvent {
                key: KeyInfo { keycode: key, .. },
                ..
            })) if key == Keycode::ESCAPE => break 'game_loop,

            // Resize the window
            Some(Event::Window(WindowEvent {
                event: WindowEventEnum::Resized { w, h },
                ..
            })) => pixels.resize_surface(w as u32, h as u32),

            _ => (),
        }

        // Update internal state
        falling_sand_state.update();

        // Draw the current frame
        falling_sand_state.draw(pixels.get_frame());
        pixels.render()?;
    }

    Ok(())
}

impl FallingSandState {
    /// Create a new `FallingSandState
    ///` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            universe:Universe::new(WIDTH as usize, HEIGHT as usize, PIXEL_SIZE as usize)
        }
    }

    /// Update the `FallingSandState
    ///` internal state; bounce the box around the screen.
    fn update(&mut self) {
        
    }

    /// Draw the `FallingSandState
    ///` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, frame: &mut [u8]) {
        let mut i = 0;
        for x in 0..self.universe.width{
            for y in 0..self.universe.height{
                if(self.universe.get_cell_at(x, y).particle_type==lib::ParticleTypes::Sand){
                    let frame_i = i * 4;
                    frame[frame_i] = 255;
                    frame[frame_i + 1] = 0;
                    frame[frame_i + 2] = 0;
                    frame[frame_i + 3] = 255;
                }
                i+=1;
            }
        }
    }
}
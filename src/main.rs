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
const WIDTH: u32 = 200;
const HEIGHT: u32 = 200;
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
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    pixels.resize_surface(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut falling_sand_state= FallingSandState::new();
    for x in 10..WIDTH-10{
        for y in 0..10 {
            falling_sand_state.universe.set_cell_at(x, y, lib::ParticleTypes::Sand,pixels.get_frame())
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
            _ => (),
        }

        // Update internal state
        falling_sand_state.update(pixels.get_frame());

        // Draw the current frame
        pixels.render().expect("Failed to render pixels");
    }

    Ok(())
}

impl FallingSandState {
    /// Create a new `FallingSandState
    ///` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            universe:Universe::new(WIDTH, HEIGHT, PIXEL_SIZE)
        }
    }

    /// Update the `FallingSandState
    ///` internal state; bounce the box around the screen.
    fn update(&mut self,frame:&mut [u8]) {
        self.universe.universe_timer=self.universe.universe_timer.wrapping_add(1);
        for x in 0..self.universe.width{
            for y in 0..self.universe.height{
                self.universe.get_cell_at(x, y).update(&mut self.universe, frame)
            }
        }

    }
}
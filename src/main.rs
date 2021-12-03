use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Text};
use ggez::event::{self, EventHandler};
use glam::*;
fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("falling_sand_game", "DucktorCid/Badfitz67")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        let fpsText = Text::new(format!("FPS:{}",ggez::timer::fps(ctx)));

        graphics::draw(ctx,&fpsText,(Vec2::new(0.0,0.0),Color::BLACK))?;

        graphics::present(ctx)
    }
}
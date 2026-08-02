// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, GraphicsContext, Image, Mesh, Rect};
use ggez::{Context, GameResult, glam};
use glam::Vec2;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

// el struct principal
struct MyGame {}

impl MyGame {
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        Ok(MyGame {})
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("HitBox", "Helio Studio Games")
        .window_setup(conf::WindowSetup::default().title("ggez :: HitBox"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(WIDTH, HEIGHT)
                .maximized(false)
                .resizable(false),
        );

    let (mut ctx, event_loop) = cb.build()?;

    //let state = MyGame::new(&mut ctx);
    let state = MyGame::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}

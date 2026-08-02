// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh};
use ggez::{Context, /*ContextBuilder,*/ GameResult};
use rand::prelude::*;
use std::mem;
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const BALL_RADIUS: f32 = 10.0;

const N: usize = 25; //max 100
//porque tienes círculos pequeños y colisiones. A más pasos:

//menos probabilidad de atravesarse,
//colisiones más estables.
const PHYSICS_DT: f32 = 1.0 / 120.0; //60

struct Ball {
    x: f32,
    y: f32,
    vx: f32, // píxeles por segundo
    vy: f32,
    hit_timer: f32,
}

impl Ball {
    // 1.
    fn new(x: f32, y: f32) -> Self {
        Self {
            x: x, //50.0,
            y: y, //50.0,
            vx: 300.0,
            vy: 180.0,
            hit_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        if self.x < 0.0 {
            self.x = -self.x;
            self.vx = -self.vx;
        } else if self.x > WIDTH {
            let exceso = self.x - WIDTH;
            self.x = WIDTH - exceso;
            self.vx = -self.vx;
        }

        if self.y < 0.0 {
            self.y = -self.y;
            self.vy = -self.vy;
        } else if self.y > HEIGHT {
            let exceso = self.y - HEIGHT;
            self.y = HEIGHT - exceso;
            self.vy = -self.vy;
        }

        if self.hit_timer > 0.0 {
            self.hit_timer -= dt;

            if self.hit_timer < 0.0 {
                self.hit_timer = 0.0;
            }
        }
        //self.hit_timer = (self.hit_timer - dt).max(0.0);
    }

    // 2.
    /*fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let color = if self.hit_timer > 0.0 {
            Color::YELLOW
        } else {
            Color::RED
        };

        let circle = Mesh::new_circle(
            ctx,
            DrawMode::fill(),          // También puede ser DrawMode::stroke(2.0)
            Vec2::new(self.x, self.y), // 200.0, 150.0 Centro
            BALL_RADIUS,               // Radio
            0.1,                       // Tolerancia
            color,                     // Color
        )?;

        canvas.draw(&circle, DrawParam::default());
        Ok(())
    }*/

    // mejora
    fn draw(&self, canvas: &mut graphics::Canvas, mesh: &Mesh) -> GameResult {
        //println!("hit_timer: {}", self.hit_timer);

        let t = (self.hit_timer / 0.5).clamp(0.0, 1.0);

        let color = Color::new(1.0, t, 0.0, 1.0);

        let param = DrawParam::default().dest(Vec2::new(self.x, self.y)).color(color);

        canvas.draw(mesh, param);

        Ok(())
    }
}
// Ball

fn handle_collisions(balls: &mut Vec<Ball>) {
    let n = balls.len();
    let min_dist = BALL_RADIUS * 2.0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (left, right) = balls.split_at_mut(j);

            let a = &mut left[i];
            let b = &mut right[0];

            let dx = b.x - a.x;
            let dy = b.y - a.y;

            let dist2 = dx * dx + dy * dy;

            if dist2 == 0.0 {
                continue;
            }

            let dist = dist2.sqrt();

            if dist < min_dist {
                // Normal de la colisión
                let nx = dx / dist;
                let ny = dy / dist;

                // Separar las pelotas para que no queden superpuestas
                let overlap = (min_dist - dist) * 0.5;

                a.x -= nx * overlap;
                a.y -= ny * overlap;

                b.x += nx * overlap;
                b.y += ny * overlap;

                // Intercambiar velocidades
                mem::swap(&mut a.vx, &mut b.vx);
                mem::swap(&mut a.vy, &mut b.vy);

                // Destello amarillo durante 150 ms
                a.hit_timer = 0.5; //0.15
                b.hit_timer = 0.5; //0.15
            }
        }
    }
}

fn draw_fps(ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
    let fps_counter = ctx.time.fps();

    let fps_text = format!("FPS: {:.0}", fps_counter);
    let mut fps_text_layout = graphics::Text::new(fps_text);
    fps_text_layout.set_scale(graphics::PxScale::from(22.0));

    // score en la parte superior izquierda
    let fps_text_position = Vec2::new(0.0, 1.0);
    canvas.draw(
        &fps_text_layout,
        DrawParam::default()
            .dest(fps_text_position)
            .color(graphics::Color::WHITE),
    );
    Ok(())
}

fn draw_ball_info(canvas: &mut graphics::Canvas, count: &usize) -> GameResult {
    let info_text = format!("Pelotas: {:.0}", count);
    let mut info_text_layout = graphics::Text::new(info_text);
    info_text_layout.set_scale(graphics::PxScale::from(22.0));

    // score en la parte superior izquierda
    let info_text_position = Vec2::new(0.0, 17.0);
    canvas.draw(
        &info_text_layout,
        DrawParam::default()
            .dest(info_text_position)
            .color(graphics::Color::WHITE),
    );
    Ok(())
}

// el struct principal
struct MyGame {
    //ball: Ball,
    balls: Vec<Ball>,
    //mejora
    ball_mesh: Mesh,
    accumulator: f32,
}

impl MyGame {
    // 1.
    pub fn new(ctx: &mut Context) -> GameResult<MyGame> {
        let mut rng = rand::rng();

        // crear las pelotas
        let balls: Vec<Ball> = (0..N)
            .map(|_| //Ball::new(), 
                Ball {
                    x: rng.random_range(0.0..WIDTH),
                    y: rng.random_range(0.0..HEIGHT),
                    vx: rng.random_range(-300.0..300.0),
                    vy: rng.random_range(-300.0..300.0),
                    hit_timer: 0.0,
                })
            .collect();

        let ball_mesh = Mesh::new_circle(ctx, DrawMode::fill(), Vec2::ZERO, BALL_RADIUS, 0.1, Color::WHITE)?;

        Ok(MyGame {
            // crear la pelota
            //ball: Ball::new(50.0, 50.0)
            balls,
            ball_mesh,
            accumulator: 0.0,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        /*
        // actualizar el movimiento de la pelota
        let dt = ctx.time.delta().as_secs_f32();

        //self.ball.update(dt);
        for ball in &mut self.balls {
            ball.update(dt);
        }
        handle_collisions(&mut self.balls);
        */
        let dt = ctx.time.delta().as_secs_f32();

        self.accumulator += dt;
        self.accumulator = self.accumulator.min(0.1);

        while self.accumulator >= PHYSICS_DT {
            for ball in &mut self.balls {
                ball.update(PHYSICS_DT);
            }

            handle_collisions(&mut self.balls);

            self.accumulator -= PHYSICS_DT;
        }

        Ok(())
    }

    // 2.
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        //self.ball.draw(ctx, &mut canvas)?;
        for ball in &self.balls {
            //ball.draw(ctx, &mut canvas)?;
            ball.draw(&mut canvas, &self.ball_mesh)?;
        }
        //self.fps_counter = ctx.time.fps();
        draw_fps(ctx, &mut canvas)?;

        let ball_counter = &self.balls.len();
        draw_ball_info(&mut canvas, ball_counter)?;

        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Bouncing Ball", "Helio Studio Games")
        .window_setup(conf::WindowSetup::default().title("ggez :: Bouncing Ball"))
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

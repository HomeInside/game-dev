use macroquad::color::{BLACK, Color, WHITE};
use macroquad::rand::gen_range;
use macroquad::shapes::draw_circle;
use macroquad::text::draw_text;
use macroquad::time::{draw_fps, get_fps, get_frame_time};
use macroquad::window;
use std::mem;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const BALL_RADIUS: f32 = 10.0;
const N: usize = 25; //max 100

const PHYSICS_DT: f32 = 1.0 / 120.0; //60

struct Ball {
    x: f32,
    y: f32,
    vx: f32, // píxeles por segundo
    vy: f32,
    hit_timer: f32,
}

impl Ball {
    fn new() -> Self {
        Self {
            x: 50.0,
            y: 50.0,
            vx: 300.0,
            vy: 180.0,
            hit_timer: 0.0,
        }
    }
    // radio visible
    #[allow(dead_code)]
    fn update1(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Rebote horizontal
        if self.x < BALL_RADIUS {
            self.x = BALL_RADIUS;
            self.vx = -self.vx;
        } else if self.x > WIDTH - BALL_RADIUS {
            self.x = WIDTH - BALL_RADIUS;
            self.vx = -self.vx;
        }

        // Rebote vertical
        if self.y < BALL_RADIUS {
            self.y = BALL_RADIUS;
            self.vy = -self.vy;
        } else if self.y > HEIGHT - BALL_RADIUS {
            self.y = HEIGHT - BALL_RADIUS;
            self.vy = -self.vy;
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
        }
    }

    fn draw(&self) {
        let t = (self.hit_timer / 0.15).clamp(0.0, 1.0);

        let color = Color::new(1.0, t, 0.0, 1.0);

        draw_circle(self.x, self.y, BALL_RADIUS, color);
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macroquad :: Bouncing Ball".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}
fn handle_collisions1(balls: &mut Vec<Ball>) {
    let n = balls.len();

    for i in 0..n {
        for j in (i + 1)..n {
            let (left, right) = balls.split_at_mut(j);

            let a = &mut left[i];
            let b = &mut right[0];

            let dx = b.x - a.x;
            let dy = b.y - a.y;

            let dist2 = dx * dx + dy * dy;
            let min_dist = BALL_RADIUS * 2.0;

            if dist2 < min_dist * min_dist {
                // Colisión
                mem::swap(&mut a.vx, &mut b.vx);
                mem::swap(&mut a.vy, &mut b.vy);
                a.hit_timer = 0.15;
                b.hit_timer = 0.15;
            }
        }
    }
}
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

#[macroquad::main(window_conf)]
async fn main() {
    let mut balls: Vec<Ball> = (0..N)
        .map(|_| Ball {
            x: gen_range(0.0, WIDTH),
            y: gen_range(0.0, HEIGHT),
            vx: gen_range(-300.0, 300.0),
            vy: gen_range(-300.0, 300.0),
            hit_timer: 0.0,
        })
        .collect();
    //
    //const PHYSICS_DT: f32 = 1.0 / 60.0;
    //let fps = get_fps().clamp(1, 1000) as f32;
    //let physics_dt = 1.0 / fps;
    //
    //let physics_dt = get_frame_time();

    let mut accumulator = 0.0;

    loop {
        accumulator += get_frame_time();

        while accumulator >= PHYSICS_DT {
            for ball in &mut balls {
                ball.update(PHYSICS_DT);
            }

            handle_collisions(&mut balls);

            accumulator -= PHYSICS_DT;
        }

        window::clear_background(BLACK);

        for ball in &balls {
            ball.draw();
        }
        draw_fps();
        draw_text(&format!("Pelotas: {}", balls.len()), 0.0, 30.0, 24.0, WHITE);

        window::next_frame().await;
    }
    /*loop {
        let dt = get_frame_time();

        for ball in &mut balls {
            ball.update(dt);
        }
        // Colisiones
        handle_collisions(&mut balls);

        window::clear_background(BLACK);

        // Dibujar todas las pelotas
        for ball in &balls {
            ball.draw();
        }

        draw_text(
            &format!("Pelotas: {}", balls.len()),
            20.0,
            30.0,
            24.0,
            WHITE,
        );
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 60.0, 30.0, GREEN);
        //draw_fps();
        window::next_frame().await;
    }*/
}

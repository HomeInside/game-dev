// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::color::{BLACK, Color, WHITE};
use macroquad::rand::gen_range;
use macroquad::shapes::draw_circle;
use macroquad::text::draw_text;
use macroquad::time::{draw_fps, get_frame_time};
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
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            x: 50.0,
            y: 50.0,
            vx: 300.0,
            vy: 180.0,
            hit_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // calculamos el movimiento: `posición = velocidad × tiempo`
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // rebote en los bordes
        // si la pelota sobrepasa algun borde (de la ventana)
        // se invierte la posición y se cambia la velocidad

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

        // si la pelota ha chocado recientemente, se reduce
        // el contador de tiempo(hit_timer) para que el
        // destello amarillo desaparezca gradualmente
        if self.hit_timer > 0.0 {
            self.hit_timer -= dt;
        }
    }

    fn draw(&self) {
        // `clamp` limita el valor (`self.hit_timer`) dentro
        // de un rango entre un mínimo(`0.0`) y un máximo(`1.0`)
        let t = (self.hit_timer / 0.15).clamp(0.0, 1.0);

        let color = Color::new(1.0, t, 0.0, 1.0);

        draw_circle(self.x, self.y, BALL_RADIUS, color);
    }
}

// detecta cuando dos pelotas se tocan y las hace rebotar
fn handle_collisions(balls: &mut Vec<Ball>) {
    let n = balls.len();
    let min_dist = BALL_RADIUS * 2.0;

    for i in 0..n {
        for j in (i + 1)..n {
            // Rust no permite dos referencias mutables al
            // mismo vector, `split_at_mut` divide el vector
            // en dos mitades mutables
            let (left, right) = balls.split_at_mut(j);

            // de esta forma se puede cambiar las propiedades
            // de `a` y `b` simultáneamente (ya que son de diferentes slices)
            let a = &mut left[i];
            let b = &mut right[0];

            // calcular distancia entre sus centros
            // usando el teorema de Pitágoras

            let dx = b.x - a.x;
            let dy = b.y - a.y;

            // distancia al cuadrado (se puede usar sqrt)
            let dist2 = dx * dx + dy * dy;

            if dist2 == 0.0 {
                continue;
            }

            let dist = dist2.sqrt();

            // si la distancia es menor, hay colisión
            if dist < min_dist {
                // la "normal" es la dirección en la que
                // ocurre el choque entre las pelotas
                // al intercambiar velocidades completas (`x` y `y`),
                // simulamos un choque elástico (casi) perfecto
                let nx = dx / dist;
                let ny = dy / dist;

                // separar las pelotas para que no queden
                // superpuestas, empujarlas en direcciones opuestas
                let overlap = (min_dist - dist) * 0.5;

                // se intercambian las velocidades

                a.x -= nx * overlap;
                a.y -= ny * overlap;

                b.x += nx * overlap;
                b.y += ny * overlap;

                // intercambiar velocidades
                // `mem::swap` intercambia los valores de dos
                // variables sin crear copia, esto requiere
                // referencias mutables (&mut).
                // Es más eficiente que hacer
                // ````
                // let temp = a.vx;
                // a.vx = b.vx;
                // b.vx = temp;
                // ````
                mem::swap(&mut a.vx, &mut b.vx);
                mem::swap(&mut a.vy, &mut b.vy);

                // destello amarillo durante 150 ms
                a.hit_timer = 0.5; //0.15
                b.hit_timer = 0.5; //0.15
            }
        }
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macroquad :: macro-ball".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // crear las pelotas
    let mut balls: Vec<Ball> = (0..N)
        .map(|_| Ball {
            x: gen_range(0.0, WIDTH),
            y: gen_range(0.0, HEIGHT),
            vx: gen_range(-300.0, 300.0),
            vy: gen_range(-300.0, 300.0),
            hit_timer: 0.0,
        })
        .collect();

    let mut accumulator = 0.0;

    loop {
        // guarda el "tiempo sobrante" para no
        // perder frames
        accumulator += get_frame_time();

        // sistema físico de paso fijo.
        // la física siempre avanza en pasos
        // iguales (según `PHYSICS_DT`)
        // independientemente de cuántos FPS
        // tenga el juego.
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
}

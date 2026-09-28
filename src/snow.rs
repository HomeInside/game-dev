/// se utilizan efectos de particulas
/// - https://github.com/not-fl3/macroquad/blob/master/examples/particles_example.rs
/// - https://es.wikipedia.org/wiki/Nieve
/// - https://es.wikipedia.org/wiki/Copo_de_nieve
///
use macroquad::prelude::*;
use macroquad::rand::gen_range;

struct SnowFlake {
    pos: Vec2,
    // profundidad visual
    depth: f32,
    // velocidad con que cae
    speed: f32,
    // radio del copo de nieve
    size: f32,

    // ancho y alto de la zona
    // de nevada
    width: f32,
    height: f32,

    color: Color,
}

impl SnowFlake {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        let depth: f32 = gen_range(0.0_f32, 1.0_f32).powf(1.5);
        Self {
            pos: vec2(gen_range(0.0, width), gen_range(-height, height)),
            depth,
            speed: 0.5,
            size: 1.0,
            width,
            height,
            color,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Caída vertical (siempre hacia abajo, nunca recta por el viento)
        self.pos.y += self.speed * dt;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.size, self.color);
    }
}

pub struct Snow {
    width: f32,
    height: f32,
    flake_color: Color,
    flakes: Vec<SnowFlake>,
}

impl Snow {
    pub fn new(width: f32, height: f32, max_flakes: usize, flake_color: Color) -> Self {
        let mut flakes = Vec::with_capacity(max_flakes);

        for _ in 0..max_flakes {
            flakes.push(SnowFlake::new(width, height, flake_color));
        }

        Self {
            width,
            height,
            flake_color,
            flakes,
        }
    }

    pub fn update(&mut self, dt: f32) {
        for f in &mut self.flakes {
            f.update(dt);
        }
    }

    pub fn draw(&self) {
        for f in &self.flakes {
            f.draw();
        }
    }
}

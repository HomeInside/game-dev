/// se utilizan efectos de particulas
/// - https://github.com/not-fl3/macroquad/blob/master/examples/particles_example.rs
/// - https://es.wikipedia.org/wiki/Nieve
///
use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub struct Snowflake {
    pos: Vec2,
    // profundidad visual
    depth: f32,
    // velocidad con que cae
    speed: f32,
    // radio del copo de nieve
    size: f32,
}

impl Snowflake {
    pub fn new(width: f32, height: f32) -> Self {
        let depth: f32 = gen_range(0.0_f32, 1.0_f32).powf(1.5);
        Self{
            ..
        }   
       
    }

    pub fn update(&mut self, dt: f32) {
        // Caída vertical (siempre hacia abajo, nunca recta por el viento)
        self.pos.y += self.speed * dt;
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.size, color);
    }
}

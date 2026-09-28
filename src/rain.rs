/// se utilizan efectos de particulas
/// - https://github.com/not-fl3/macroquad/blob/master/examples/particles_example.rs
///
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use macroquad::window::{self, next_frame};

struct Raindrop {
    pos: Vec2,
    // profundidad visual
    depth: f32,
    // velocidad con que cae
    speed: f32,
    // La longitud de la linea
    length: f32,

    // el viento, hace que cada
    // linea tenga inclinación
    wind: f32,

    // ancho y alto de la zona
    // de lluvia
    width: f32,
    height: f32,

    rain_color: Color,
}

impl Raindrop {
    fn get_measures() -> (f32, f32, f32) {
        //let depth = gen_range(0.0, 1.0);
        let depth: f32 = gen_range(0.0_f32, 1.0_f32).powf(1.5_f32);

        // la profundidad visual controla la velocidad,
        // entre mas lejos mas rapido
        let speed = match depth {
            /*
            d if d < 0.35 => gen_range(360.0, 520.0), //cerca
            d if d < 0.75 => gen_range(200.0, 340.0), //media
            _ => gen_range(100.0, 190.0),             //lejos
            */
            d if d < 0.35 => gen_range(200.0, 360.0), //lejos
            d if d < 0.75 => gen_range(360.0, 450.0), //media
            _ => gen_range(460.0, 600.0),             //cerca
        };

        // La longitud de la linea que dibuja la gota
        // también depende de la profundidad.
        let length = match depth {
            d if d < 0.35 => gen_range(4.0, 7.0),  //3
            d if d < 0.75 => gen_range(8.0, 12.0), //4
            _ => gen_range(13.0, 19.0),            // 6
        };
        (depth, speed, length)
    }

    pub fn new(width: f32, height: f32, rain_color: Color) -> Self {
        let get_measures = Self::get_measures();

        Self {
            pos: vec2(gen_range(0.0, width), gen_range(0.0, height)),
            depth: get_measures.0,
            speed: get_measures.1,
            length: get_measures.2,
            // el viento es variable para cada gota
            wind: gen_range(0.75, 1.25),
            width,
            height,
            rain_color,
        }
    }

    fn get_wind_speed(&self) -> f32 {
        let time = get_time() as f32;

        // inclinacion ligera
        let wind_speed = -45.0 + (time * 0.35).sin() * 12.0 + (time * 0.17).sin() * 6.0;

        // "lluvia perfecta", como paseo estelar
        //let wind_speed = -2.0 + (time * 0.35).sin() * 1.0 + (time * 0.17).sin() * 0.5;

        // lluvia más tranquila
        //let wind_speed = -35.0;

        // tipo 2D:
        //let wind_speed = -60.0;

        // tormenta
        //let wind_speed = -200.0;
        wind_speed
    }

    pub fn update(&mut self, dt: f32) {
        let wind_speed = self.get_wind_speed();

        let vx = wind_speed * self.wind;

        self.pos.x += vx * dt;
        self.pos.y += self.speed * dt;

        // si sale por abajo, reaparece arriba
        if self.pos.y > self.height + self.length {
            self.pos.y = -self.length;

            self.pos.x = gen_range(-20.0, self.width + 20.0);
        }

        // si el viento la lleva demasiado lejos
        // a la izquierda
        if self.pos.x < -30.0 {
            self.pos.x = self.width + 10.0;
        }

        // si el viento la lleva demasiado lejos
        // a la derecha
        if self.pos.x > self.width + 30.0 {
            self.pos.x = -10.0;
        }
    } //update

    pub fn draw(&self) {
        let wind_speed = self.get_wind_speed();

        let vx = wind_speed * self.wind;
        let vy = self.speed;

        let direction = vec2(vx, vy).normalize();

        // La estela representa el movimiento.
        let trail = direction * self.length;

        let (alpha, thickness) = if self.depth < 0.35 {
            (0.25, 0.7) // lejos
        } else if self.depth < 0.75 {
            (0.55, 1.0) // media
        } else {
            (0.85, 1.5) // cerca
        };

        draw_line(
            self.pos.x,
            self.pos.y,
            self.pos.x - trail.x,
            self.pos.y - trail.y,
            thickness,
            self.rain_color,
        );

        if self.depth > 0.25 {
            draw_circle(self.pos.x, self.pos.y, thickness * 0.55, self.rain_color);
        }
    }
}

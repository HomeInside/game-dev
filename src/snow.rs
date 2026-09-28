/// se utilizan efectos de particulas
/// - https://github.com/not-fl3/macroquad/blob/master/examples/particles_example.rs
/// - https://es.wikipedia.org/wiki/Nieve
/// - https://es.wikipedia.org/wiki/Copo_de_nieve
///
use macroquad::prelude::*;
use macroquad::rand::gen_range;

#[derive(Clone, Copy)]
enum SnowType {
    Tiny,
    Normal,
    Flake,
}

// algunos valores prefijados
// para los copos de nieve
struct Measures {
    depth: f32,
    speed: f32,
    size: f32,
    kind: SnowType,
    alpha: f32,
    color: Color,
}

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

    // rotación para los copos tipo "Flake"
    rotation: f32,

    alpha_color: f32,
    color: Color,
    kind: SnowType,
}

impl SnowFlake {
    fn get_measures() -> Measures {
        let depth: f32 = gen_range(0.0_f32, 1.0_f32).powf(1.5_f32);

        // la profundidad visual controla la velocidad,
        // entre mas lejos mas rapido
        let speed = match depth {
            d if d < 0.35 => gen_range(15.0, 30.0), //lejos
            d if d < 0.75 => gen_range(30.0, 55.0), //media
            _ => gen_range(55.0, 90.0),             //cerca
        };

        // el tamaño de cada copo de nieve también
        // depende de la profundidad.
        let size = match depth {
            d if d < 0.35 => gen_range(1.0, 1.8),
            d if d < 0.75 => gen_range(1.9, 2.8),
            _ => gen_range(2.9, 4.0),
        };

        // la profundidad visual permite cambiar el
        // tipo de copo de nieve
        let kind = match depth {
            d if d < 0.35 => SnowType::Tiny,
            d if d < 0.75 => SnowType::Normal,
            _ => SnowType::Flake,
        };

        // el alpha para el color según el tipo
        let alpha = match kind {
            SnowType::Tiny => 0.45,
            SnowType::Normal => 0.75,
            SnowType::Flake => 0.95,
        };

        let color = Color::new(1.0, 1.0, 1.0, alpha);

        Measures {
            depth,
            speed,
            size,
            kind,
            alpha,
            color,
        }
    }

    pub fn new(width: f32, height: f32) -> Self {
        let depth: f32 = gen_range(0.0_f32, 1.0_f32).powf(1.5);
        let get_measures = Self::get_measures();

        Self {
            pos: vec2(gen_range(0.0, width), gen_range(-height, height)),
            depth: get_measures.depth,
            speed: get_measures.speed,
            size: get_measures.size,
            width,
            height,
            rotation: gen_range(0.0, std::f32::consts::TAU),
            alpha_color: get_measures.alpha,
            color: get_measures.color,
            kind: get_measures.kind,
        }
    }

    fn get_wind_speed(&self) -> f32 {
        let time = get_time() as f32;

        let wind_speed = 15.0 + (time * 0.2).sin() * 10.0;

        wind_speed
    }

    pub fn update(&mut self, dt: f32) {
        let wind_speed = self.get_wind_speed();

        //let vx = wind_speed * self.wind;
        let vx = wind_speed * dt;

        self.pos.y += self.speed * dt;

        self.pos.x += vx;

        // si sale por abajo, reaparece arriba
        if self.pos.y > self.height + self.size {
            self.pos.y = -self.size * 2.0;

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
    }

    pub fn draw(&self) {
        match self.kind {
            SnowType::Tiny => {
                draw_circle(self.pos.x, self.pos.y, self.size, self.color);
            }
            SnowType::Normal => {
                draw_circle(self.pos.x, self.pos.y, self.size, self.color);
                // pequeño halo para dar volumen
                draw_circle_lines(
                    self.pos.x,
                    self.pos.y,
                    self.size + 0.5,
                    0.5,
                    Color::new(1.0, 1.0, 1.0, self.alpha_color * 0.4),
                );
            }
            SnowType::Flake => {
                // copo con forma de cruz
                let arm = self.size * 1.5;
                let c = self.rotation.cos();
                let s = self.rotation.sin();

                // brazo horizontal
                draw_line(
                    self.pos.x - arm * c,
                    self.pos.y - arm * s,
                    self.pos.x + arm * c,
                    self.pos.y + arm * s,
                    1.0,
                    self.color,
                );

                // brazo vertical
                draw_line(
                    self.pos.x + arm * s,
                    self.pos.y - arm * c,
                    self.pos.x - arm * s,
                    self.pos.y + arm * c,
                    1.0,
                    self.color,
                );

                // centro
                draw_circle(self.pos.x, self.pos.y, self.size * 0.4, self.color);
            }
        }
    }
}

pub struct Snow {
    width: f32,
    height: f32,
    flakes: Vec<SnowFlake>,
}

impl Snow {
    pub fn new(width: f32, height: f32, max_flakes: usize) -> Self {
        let mut flakes = Vec::with_capacity(max_flakes);

        for _ in 0..max_flakes {
            flakes.push(SnowFlake::new(width, height));
        }

        Self { width, height, flakes }
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

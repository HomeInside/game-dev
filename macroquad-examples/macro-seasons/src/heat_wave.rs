/// se utilizan efectos de particulas
/// - https://github.com/not-fl3/macroquad/blob/master/examples/particles_example.rs
/// - https://es.wikipedia.org/wiki/Espejismo
///
/// El espejismo de aire caliente, también llamada brillo de calor,
/// ("heat haze", "heat shimmer", "mirage") se refiere al espejismo inferior
/// observado cuando se ven objetos a través de una masa de aire caliente.
///
use macroquad::prelude::*;
use macroquad::rand::gen_range;

#[derive(Clone, Copy)]
enum HazeKind {
    Shimmer, // línea ondulada vertical fina
    Blob,    // círculo difuso grande
    Wisp,    // "tira" alargado (línea gruesa vertical)
}

struct HeatWave {
    pos: Vec2,
    speed: f32,  // velocidad vertical
    size: f32,   // grosor / radio
    length: f32, // largo para Shimmer / Wisp
    width: f32,
    height: f32,
    phase: f32, // desfase para turbulencia
    turb: f32,  // amplitud de la turbulencia lateral
    alpha: f32,
    color: Color,
    kind: HazeKind,
    life: f32,
    life_speed: f32,
}

impl HeatWave {
    pub fn new(width: f32, ground_y: f32, height: f32) -> Self {
        // aparecen solo en la franja de la carretera
        let road_h = height - ground_y;

        // inician en el asfalto, un poco por encima del borde inferior
        let y = gen_range(ground_y + road_h * 0.4, height - 4.0);

        let kind = match gen_range(0, 10) {
            0..=4 => HazeKind::Shimmer,
            5..=7 => HazeKind::Blob,
            _ => HazeKind::Wisp,
        };

        let (size, length, alpha) = match kind {
            HazeKind::Shimmer => (gen_range(0.8, 1.6), gen_range(20.0, 55.0), gen_range(0.05, 0.15)),
            HazeKind::Blob => (gen_range(20.0, 45.0), 0.0, gen_range(0.03, 0.08)),
            HazeKind::Wisp => (gen_range(3.0, 6.0), gen_range(30.0, 70.0), gen_range(0.04, 0.10)),
        };

        // el calor sube
        let speed = gen_range(25.0, 90.0);

        Self {
            pos: vec2(gen_range(0.0, width), y),
            speed,
            size,
            length,
            width,
            height,
            phase: gen_range(0.0, std::f32::consts::TAU),
            turb: gen_range(10.0, 30.0),
            alpha,
            // blanco cálido: ligeramente más rojo que azul
            color: Color::new(1.0, 0.92, 0.58, alpha),
            kind,
            life: 0.0,
            life_speed: gen_range(0.15, 0.35),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let time = get_time() as f32;

        // sube
        self.pos.y -= self.speed * dt;

        // el viento afecta el tipo de onda
        self.pos.x += (time * 2.0 + self.phase).sin() * self.turb * dt;

        self.life += self.life_speed * dt;

        // si sale por arriba, reaparece abajo
        if self.life >= 1.0 || self.pos.y < self.height * 0.35 {
            self.reset();
        }
    }

    fn reset(&mut self) {
        let ground_y = self.height - 80.0; // aprox, se ajusta con el campo real
        let road_h = self.height - ground_y;

        self.pos = vec2(
            gen_range(0.0, self.width),
            gen_range(self.height - road_h * 0.3, self.height - 2.0),
        );

        self.life = 0.0;
        self.phase = gen_range(0.0, std::f32::consts::TAU);

        self.alpha = match self.kind {
            HazeKind::Shimmer => gen_range(0.05, 0.15),
            HazeKind::Blob => gen_range(0.03, 0.08),
            HazeKind::Wisp => gen_range(0.04, 0.10),
        };

        self.color = Color::new(1.0, 0.92, 0.78, self.alpha);
    }

    pub fn draw(&self) {
        // desvanecimiento
        let fade = {
            let t = self.life;
            if t < 0.15 { t / 0.15 } else { 1.0 - (t - 0.15) / 0.85 }
        }
        .clamp(0.0, 1.0);

        let a = self.alpha * fade;
        let col = Color::new(self.color.r, self.color.g, self.color.b, a);

        match self.kind {
            HazeKind::Shimmer => {
                // en lugar de dibujar una sola línea recta, la
                // dividimos en segmentos y la desplazamos

                let segments = 6;
                let step = self.length / segments as f32;

                let mut prev = self.pos;

                for i in 1..=segments {
                    let t = i as f32 / segments as f32;

                    let wave = (t * 6.0 + self.phase + get_time() as f32 * 4.0).sin() * self.turb * 0.15;
                    let p = vec2(self.pos.x + wave, self.pos.y - step * i as f32);

                    draw_line(prev.x, prev.y, p.x, p.y, self.size, col);
                    prev = p;
                }
            }
            HazeKind::Blob => {
                // tipo círculo muy difuso

                for i in 0..3 {
                    let r = self.size * (1.0 - i as f32 * 0.25);
                    let a2 = a * (0.4 - i as f32 * 0.1).max(0.0);

                    draw_circle(self.pos.x, self.pos.y, r, Color::new(col.r, col.g, col.b, a2));
                }
            }
            HazeKind::Wisp => {
                // línea gruesa que va cambiando de grosor y movimiento
                // se va afinando y ondulando

                let segments = 5;
                let step = self.length / segments as f32;
                let mut prev = self.pos;

                for i in 1..=segments {
                    let t = i as f32 / segments as f32;
                    let wave = (t * 4.0 + self.phase + get_time() as f32 * 3.0).sin() * self.turb * 0.2;
                    let p = vec2(self.pos.x + wave, self.pos.y - step * i as f32);

                    let w = self.size * (1.0 - t * 0.7);

                    draw_line(prev.x, prev.y, p.x, p.y, w, col);
                    prev = p;
                }
            }
        }
    }
}

pub struct HeatHaze {
    width: f32,
    height: f32,
    ground_y: f32,
    waves: Vec<HeatWave>,
}

impl HeatHaze {
    pub fn new(width: f32, height: f32, count: usize) -> Self {
        let ground_y = height * 0.76;

        let mut waves = Vec::with_capacity(count);

        for _ in 0..count {
            waves.push(HeatWave::new(width, ground_y, height));
        }

        Self {
            width,
            height,
            ground_y,
            waves,
        }
    }

    pub fn update(&mut self, dt: f32) {
        for w in &mut self.waves {
            w.update(dt);
        }
    }

    pub fn draw(&self) {
        for w in &self.waves {
            w.draw();
        }
    }
}

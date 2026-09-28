/// se utilizan efectos de particulas
/// - https://github.com/not-fl3/macroquad/blob/master/examples/particles_example.rs
/// - https://es.wikipedia.org/wiki/Nieve
/// - https://es.wikipedia.org/wiki/Copo_de_nieve
///
use macroquad::prelude::*;
use macroquad::rand::gen_range;

//se crean varios tipos de copos
// de nieve para mejorar el aspecto
// visual
#[derive(Clone, Copy)]
enum SnowType {
    Tiny,
    Normal,
    Flake,
    Star,
    HexPlate,
    Dendritic,
    Rosette,
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
    // desfase para la oscilación individual
    phase: f32,

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
            d if d < 0.35 => gen_range(15.0, 30.0),
            d if d < 0.75 => gen_range(30.0, 55.0),
            _ => gen_range(55.0, 90.0),
        };

        // el tamaño de cada copo de nieve también
        // depende de la profundidad.
        let size = match depth {
            d if d < 0.35 => gen_range(1.0, 1.8),
            d if d < 0.75 => gen_range(1.9, 2.8),
            _ => gen_range(2.9, 4.0),
        };

        let kind = match gen_range(0, 7) {
            0 => SnowType::Tiny,
            1 => SnowType::Normal,
            2 => SnowType::Flake,
            3 => SnowType::Star,
            4 => SnowType::HexPlate,
            5 => SnowType::Dendritic,
            _ => SnowType::Rosette,
        };

        // el alpha para el color según el tipo
        let alpha = match depth {
            d if d < 0.35 => 0.45,
            d if d < 0.75 => 0.75,
            _ => 0.95,
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
            phase: gen_range(0.0, std::f32::consts::TAU),
            alpha_color: get_measures.alpha,
            color: get_measures.color,
            kind: get_measures.kind,
        }
    }

    fn get_wind_speed(&self) -> (f32, f32) {
        let time = get_time() as f32;

        let wind_speed = 15.0 + (time * 0.2).sin() * 10.0;

        (wind_speed, time)
    }

    pub fn update(&mut self, dt: f32) {
        let (wind_speed, time) = self.get_wind_speed();

        //let vx = wind_speed * self.wind;
        let vx = wind_speed * dt;

        self.pos.y += self.speed * dt;

        self.pos.x += (time * 1.5 + self.phase).sin() * 15.0 * dt; //10.0

        //self.pos.x += wind * dt;
        self.pos.x += vx;

        // rotación lenta para los copos tipo "Flake"
        self.rotation += 0.5 * dt;

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
                // un círculo pequeño

                draw_circle(self.pos.x, self.pos.y, self.size - 1.0, self.color);
            }
            SnowType::Normal => {
                // un círculo con un halo

                draw_circle(self.pos.x, self.pos.y, self.size - 0.8, self.color);
                // pequeño halo para dar volumen
                draw_circle_lines(
                    self.pos.x,
                    self.pos.y,
                    self.size + 0.5,
                    0.7,
                    Color::new(1.0, 1.0, 1.0, self.alpha_color * 0.4),
                );
            }
            SnowType::Flake => {
                // copo con forma de cruz

                let arm = self.size * 2.0;
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
            SnowType::Star => {
                // copo en forma de estrella de seis puntas

                let arm = self.size * 2.0;

                for i in 0..6 {
                    let angle = self.rotation + i as f32 * std::f32::consts::TAU / 6.0;

                    let direction = vec2(angle.cos(), angle.sin());

                    let end = self.pos + direction * arm;

                    draw_line(self.pos.x, self.pos.y, end.x, end.y, 1.0, self.color);
                }
            }
            SnowType::HexPlate => {
                // copo en forma de hexagono

                let radius = self.size * 1.7;

                let mut points = [Vec2::ZERO; 6];

                for i in 0..6 {
                    let angle = self.rotation + i as f32 * std::f32::consts::TAU / 6.0;

                    points[i] = self.pos + vec2(angle.cos() * radius, angle.sin() * radius);
                }

                for i in 0..6 {
                    let next = (i + 1) % 6;

                    draw_line(
                        points[i].x,
                        points[i].y,
                        points[next].x,
                        points[next].y,
                        1.0,
                        self.color,
                    );
                }
            }
            SnowType::Dendritic => {
                // copo en forma de estrella con muchas
                // ramificaciones parecidas a las ramas
                // de un árbol

                let arm = self.size * 3.0;
                let branch_angle = 30.0_f32.to_radians();

                for i in 0..6 {
                    let angle = self.rotation + i as f32 * std::f32::consts::TAU / 6.0;

                    let direction = vec2(angle.cos(), angle.sin());

                    // brazo principal
                    let end = self.pos + direction * arm;

                    draw_line(self.pos.x, self.pos.y, end.x, end.y, 1.0, self.color);

                    // ramificaciones
                    for factor in [0.35, 0.55, 0.75] {
                        let branch_pos = self.pos + direction * arm * factor;
                        let branch_length = arm * (0.18 * (1.0 - factor));

                        let angle_a = angle + branch_angle;
                        let angle_b = angle - branch_angle;

                        let dir_a = vec2(angle_a.cos(), angle_a.sin());

                        let dir_b = vec2(angle_b.cos(), angle_b.sin());

                        let end_a = branch_pos + dir_a * branch_length;
                        let end_b = branch_pos + dir_b * branch_length;

                        draw_line(branch_pos.x, branch_pos.y, end_a.x, end_a.y, 1.0, self.color);

                        draw_line(branch_pos.x, branch_pos.y, end_b.x, end_b.y, 1.0, self.color);
                    }
                }
            }
            SnowType::Rosette => {
                // un copo con varios cristales alargados o
                // prismáticos que crecen desde un punto central,
                // como los pétalos de una flor

                let radius = self.size * 3.5;
                let crystal_width = self.size * 1.2;

                for i in 0..6 {
                    let angle = self.rotation + i as f32 * std::f32::consts::TAU / 6.0;

                    let direction = vec2(angle.cos(), angle.sin());

                    let perpendicular = vec2(-direction.y, direction.x);

                    let start = self.pos + direction * self.size * 0.3;
                    let end = self.pos + direction * radius;

                    let thickness = 1.0;

                    // cristal principal
                    draw_line(start.x, start.y, end.x, end.y, thickness, self.color);

                    // pequeño grosor del cristal
                    draw_line(
                        start.x + perpendicular.x * crystal_width,
                        start.y + perpendicular.y * crystal_width,
                        end.x + perpendicular.x * crystal_width * 0.3,
                        end.y + perpendicular.y * crystal_width * 0.3,
                        thickness,
                        self.color,
                    );

                    draw_line(
                        start.x - perpendicular.x * crystal_width,
                        start.y - perpendicular.y * crystal_width,
                        end.x - perpendicular.x * crystal_width * 0.3,
                        end.y - perpendicular.y * crystal_width * 0.3,
                        thickness,
                        self.color,
                    );
                }

                // centro de la roseta
                draw_circle(self.pos.x, self.pos.y, self.size * 0.5, self.color);
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

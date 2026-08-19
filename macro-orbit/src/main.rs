// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

enum OrbiterShape {
    Rectangle,
    Circle,
}

/// Cuerpo Central
/// cuerpo cuya gravedad domina el movimiento
/// del objeto que está orbitándolo.
///
struct CentralBody {
    pos: Vec2,
    size: Vec2,
}

impl CentralBody {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
            size: vec2(width, height),
        }
    }

    pub fn update() {}

    pub fn draw(&self) {
        draw_rectangle(
            self.pos.x - self.size.x / 2.0,
            self.pos.y - self.size.y / 2.0,
            self.size.x,
            self.size.y,
            RED,
        );
    }
}

/// Orbitante
// objeto que se mueve alrededor de otro
// debido a la gravedad.
struct Orbiter {
    // parámetros orbitales
    radio_x: f32,
    radio_y: f32,
    speed: f32,
    angle: f32,
    color: Color,
    // círculo: radio, rectángulo: lado
    orbiter_size: f32,
    shape: OrbiterShape,
    show_orbit: bool,
    // esta será calculada
    pos: Vec2,
}

/// el orbitante puede ser cualquier figura
/// para este caso implementamos rectangulo y circulos
impl Orbiter {
    /// rectangulos
    pub fn new_rect(radio_x: f32, radio_y: f32, speed: f32, color: Color, orbiter_size: f32, show_orbit: bool) -> Self {
        Self {
            radio_x,
            radio_y,
            speed,
            angle: 0.0,
            color,
            orbiter_size,
            shape: OrbiterShape::Rectangle,
            show_orbit,
            //pos: vec2(0.0_f32.cos() * radio_x, 0.0_f32.sin() * radio_y),
            pos: vec2(radio_x, 0.0),
        }
    }

    ///circulos
    pub fn new_circle(
        radio_x: f32,
        radio_y: f32,
        speed: f32,
        color: Color,
        orbiter_size: f32,
        show_orbit: bool,
    ) -> Self {
        Self {
            radio_x,
            radio_y,
            speed,
            angle: 0.0,
            color,
            orbiter_size,
            shape: OrbiterShape::Circle,
            show_orbit,
            pos: vec2(radio_x, 0.0),
        }
    }

    // calcula la posición orbital para que sea
    // más tipo elipse para nuestro ejemplo
    pub fn update(&mut self, centro: Vec2, dt: f32) {
        self.angle += self.speed * dt;
        let offset = vec2(self.angle.cos() * self.radio_x, self.angle.sin() * self.radio_y);
        self.pos = centro + offset;
    }

    // se dibuja la figura segun su tipo
    pub fn draw(&self) {
        match &self.shape {
            OrbiterShape::Rectangle => {
                draw_rectangle(
                    self.pos.x - self.orbiter_size / 2.0,
                    self.pos.y - self.orbiter_size / 2.0,
                    self.orbiter_size,
                    self.orbiter_size,
                    self.color,
                );
            }
            OrbiterShape::Circle => {
                draw_circle(self.pos.x, self.pos.y, self.orbiter_size, self.color);
            }
        }

        // circulo en el centro del orbitador
        draw_circle(self.pos.x, self.pos.y, 3.0, BLACK);
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macro::orbit".to_owned(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), macroquad::Error> {
    // el cuerpo central se ubica en el centro de la ventana
    // cada objeto orbitante, crea un orbita desde el centro
    // del cuerpo central.
    //
    // para esto objetos crearemos una orbita eliptica y no circular.
    //

    let central_box = CentralBody::new(120.0, 80.0);

    // más cerca del centro de `central_box`
    let mut blue_box = Orbiter::new_rect(150.0, 150.0, 2.0, BLUE, 30.0, true);

    // más alejado del centro de `central_box`
    let mut yellow_circle = Orbiter::new_circle(300.0, 200.0, 0.8, YELLOW, 20.0, true);

    loop {
        clear_background(WHITE);

        let dt = get_frame_time();

        central_box.draw();
        //
        blue_box.update(central_box.pos, dt);
        blue_box.draw();
        //
        yellow_circle.update(central_box.pos, dt);
        yellow_circle.draw();

        next_frame().await;
    }
}

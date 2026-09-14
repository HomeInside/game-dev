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
    pub fn update(&mut self, center: Vec2, dt: f32) {
        self.angle += self.speed * dt;
        let offset = vec2(self.angle.cos() * self.radio_x, self.angle.sin() * self.radio_y);
        self.pos = center + offset;
    }

    // se dibuja la figura segun su tipo y la orbita
    pub fn draw(&self, center: Vec2, show_orbit: bool) {
        // para todos los objetos, se dibuja una elipse
        // aquí se puede validar que tipo de objeto en particular
        // es, y decidir dibujar una elipse ó un circulo
        if show_orbit && self.show_orbit {
            self.draw_ellipse(center);
        }

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

    // la función `draw_ellipse_lines` de Macroquad, no permite
    // enviar el número de segmentos, asi que usamos una variación
    // de la función original, con segmentos (líneas limitada por dos puntos),
    // calculando varios puntos alrededor y luego conectandolos
    // para simular la elipse.
    // entre más segmentos, más "definida" parecerá la elipse, con
    // pocos segmentos se vería como un polígono.
    fn draw_ellipse(&self, center: Vec2) {
        // función de Macroquad
        // draw_ellipse_lines(center.x, center.y, self.radio_x, self.radio_y, 0.0, 1.5, GRAY);

        //
        //nuestra aproximación
        //
        let total_segments: usize = 96;
        let step = 2.0 * std::f32::consts::PI / total_segments as f32;

        for i in 0..total_segments {
            // ángulo inicial del segmento
            let a1 = i as f32 * step;
            // ángulo final del segmento
            let a2 = (i + 1) as f32 * step;

            // calculo de la elipse
            let p1 = vec2(center.x + self.radio_x * a1.cos(), center.y + self.radio_y * a1.sin());
            let p2 = vec2(center.x + self.radio_x * a2.cos(), center.y + self.radio_y * a2.sin());

            draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, GRAY);
        }
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
    // el cuerpo central, se ubica en el centro de la ventana
    // cada objeto orbitante, crea un orbita desde el centro
    // del cuerpo central.
    //
    // para estos objetos crearemos una orbita eliptica y no circular.
    //

    let mut central_box = CentralBody::new(120.0, 80.0);

    // más cerca del centro de `central_box`
    let mut blue_box = Orbiter::new_rect(150.0, 150.0, 2.0, BLUE, 30.0, true);

    // más alejado del centro de `central_box`
    let mut yellow_circle = Orbiter::new_circle(300.0, 200.0, 0.8, YELLOW, 20.0, true);
    let mut show_orbit: bool = false;

    loop {
        clear_background(WHITE);

        let dt = get_frame_time();

        // mover central_box libremente

        if is_key_down(KeyCode::Right) {
            central_box.pos.x += 200.0 * dt;
        }

        if is_key_down(KeyCode::Left) {
            central_box.pos.x -= 200.0 * dt;
        }

        if is_key_down(KeyCode::Up) {
            central_box.pos.y -= 200.0 * dt;
        }

        if is_key_down(KeyCode::Down) {
            central_box.pos.y += 200.0 * dt;
        }

        // mostrar orbita
        if is_key_pressed(KeyCode::O) {
            show_orbit = !show_orbit;
        }

        central_box.draw();
        //
        blue_box.update(central_box.pos, dt);
        blue_box.draw(central_box.pos, show_orbit);
        //
        yellow_circle.update(central_box.pos, dt);
        yellow_circle.draw(central_box.pos, show_orbit);

        next_frame().await;
    }
}

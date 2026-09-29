use macroquad::prelude::*;

pub struct Summer {
    // ancho y alto de la zona
    // de nevada
    width: f32,
    height: f32,
    sun: Vec2,
    radius: f32,
    ground_y: f32,
}

impl Summer {
    fn tiled_to_circle(x: f32, y: f32, wh: f32) -> (Vec2, f32) {
        // el `width` y el `height` debe ser el mismo valor
        // para un círculo
        // se calcula el radio `r = wh/2`
        // y para `x` y `y` que representan la esquina
        // superior izquierda, el centro sería:
        // `center_x= x+r`
        // `center_y= y+r`

        let radius = wh / 2.0;
        let center = vec2(x + radius, y + radius);

        (center, radius)
    }

    pub fn new(width: f32, height: f32) -> Self {
        let (sun, radius) = Self::tiled_to_circle(740.36, 39.18, 230.55);
        let ground_y = height * 0.76;

        Self {
            width,
            height,
            sun,
            radius,
            ground_y,
        }
    }

    pub fn update(&mut self, dt: f32) {}

    pub fn draw_sun(&self, dt: f32) {
        // se crean rayos detrás de los halos y del sol,
        // una linea cada `i` ángulos
        for i in 0..16 {
            let angle = i as f32 * (2.0 * std::f32::consts::PI / 16.0);

            let start = 62.0;

            // para dar efecto de movimiento
            let end = 122.0 + dt;

            let start = self.sun + vec2(angle.cos(), angle.sin()) * start;
            let end = self.sun + vec2(angle.cos(), angle.sin()) * end;

            draw_line(start.x, start.y, end.x, end.y, 2.0, Color::new(1.0, 0.9, 0.35, 0.38));
        }

        // externo
        draw_circle(self.sun.x, self.sun.y, self.radius, Color::new(1.0, 0.82, 0.25, 0.08));

        // medio
        draw_circle(
            self.sun.x,
            self.sun.y,
            self.radius - 30.0,
            Color::new(1.0, 0.88, 0.30, 0.14),
        );

        // interno
        draw_circle(
            self.sun.x,
            self.sun.y,
            self.radius - 50.0,
            Color::new(1.0, 0.90, 0.35, 1.0),
        );
    }

    pub fn draw(&self, dt: f32) {
        self.draw_sun(dt);
    }
}

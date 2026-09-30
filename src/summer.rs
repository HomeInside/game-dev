/// se utilizan efectos de particulas
/// - https://github.com/not-fl3/macroquad/blob/master/examples/particles_example.rs
/// - https://docs.rs/macroquad/latest/macroquad/texture/fn.render_target.html
/// - https://docs.rs/macroquad/latest/macroquad/texture/struct.Texture2D.html
///
/// - https://github.com/HomeInside/game-dev/
/// - https://github.com/HomeInside/game-dev/blob/master/macroquad-examples/macro-background/src/main.rs
use macroquad::prelude::*;

pub struct Summer {
    // ancho y alto de la zona
    // de nevada
    width: f32,
    height: f32,
    sun: Vec2,
    radius: f32,
    sky: Texture2D,
    // actua como suelo/piso
    ground_y: f32,
    sun_img: Texture2D,
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

    pub async fn new(width: f32, height: f32) -> Self {
        let (sun, radius) = Self::tiled_to_circle(740.36, 39.18, 230.55);
        let ground_y = height * 0.76;
        let sky = Self::draw_sky(width as u16, height as u16);
        let sun_img = load_texture("sun1.png").await.unwrap();

        Self {
            width,
            height,
            sun,
            radius,
            sky,
            ground_y,
            sun_img,
        }
    }

    pub fn update(&mut self, dt: f32) {}

    // el sol, halo y rayos
    pub fn draw_sun(&self, dt: f32) {
        let time = get_time() as f32;

        // creamos un efecto del movimiento cada halo
        let halo_mov = (time * 2.0).sin() * 4.0;

        // se crean rayos detrás de los halos y del sol,
        // una linea cada `i` ángulos
        for i in 0..16 {
            let angle = i as f32 * (2.0 * std::f32::consts::PI / 16.0);

            let start = 82.0; //62.0;

            // para dar efecto de movimiento
            let end = 172.0 + halo_mov;

            let start = self.sun + vec2(angle.cos(), angle.sin()) * start;
            let end = self.sun + vec2(angle.cos(), angle.sin()) * end;

            draw_line(start.x, start.y, end.x, end.y, 2.0, Color::new(1.0, 0.9, 0.35, 0.38));
        }

        // halo externo
        draw_circle(
            self.sun.x,
            self.sun.y,
            self.radius + halo_mov,
            Color::new(1.0, 0.82, 0.25, 0.08),
        );

        let img_size = self.radius * 2.0;
        let pos = self.sun - vec2(img_size, img_size) * 0.5;

        // colocamos la imagen en el centro del circulo
        draw_texture_ex(
            &self.sun_img,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(img_size, img_size)),
                ..Default::default()
            },
        );

        // halo medio
        draw_circle(
            self.sun.x,
            self.sun.y,
            (self.radius + 50.0) + halo_mov,
            //self.radius + halo_mov,
            Color::new(1.0, 0.88, 0.30, 0.14),
        );

        // halo interno
        draw_circle(
            self.sun.x,
            self.sun.y,
            (self.radius + 20.0) + halo_mov,
            Color::new(1.0, 0.88, 0.30, 0.14),
        );
    }

    // calcula un valor intermedio entre a y b.
    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    // es cielo tipo Texture2D
    fn draw_sky(width: u16, height: u16) -> Texture2D {
        let color_top = Color::new(0.18, 0.52, 0.88, 1.0);
        let color_bottom = Color::new(0.58, 0.82, 0.98, 1.0);

        let mut image = Image::gen_image_color(width, height, color_top);

        // se crean N colores(usando `height` como alto),
        // para hacer el degradado
        for y in 0..height {
            // creamos un valor entre 0.0 y (`height` -1)
            // para el valor intermedio y generar el degradado
            let t = if height > 1 {
                y as f32 / (height - 1) as f32
            } else {
                0.0
            };

            let color = Color::new(
                Self::lerp(color_top.r, color_bottom.r, t),
                Self::lerp(color_top.g, color_bottom.g, t),
                Self::lerp(color_top.b, color_bottom.b, t),
                1.0,
            );

            // se rellena la imagen usando el color generado
            for x in 0..width {
                image.set_pixel(x as u32, y as u32, color);
            }
        }

        let sky_texture = Texture2D::from_image(&image);
        // se usa `Linear` para suavizar la imagen final
        sky_texture.set_filter(FilterMode::Linear);

        sky_texture
    }

    pub fn draw(&self, dt: f32) {
        draw_texture(&self.sky, 0.0, 0.0, WHITE);

        self.draw_sun(dt);
    }
}

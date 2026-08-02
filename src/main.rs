use macroquad::color::{BLACK, WHITE};
use macroquad::math::Rect;
use macroquad::prelude::*;
use macroquad::time::{draw_fps, get_frame_time};
use macroquad::window;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

#[allow(dead_code)]
const PHYSICS_DT: f32 = 1.0 / 120.0; //60

struct Dino {
    image: Texture2D,
    w: f32,
    h: f32,
    speed: Vec2,
    position: Vec2,
    hitbox: Rect,
}

impl Dino {
    pub fn new(image: Texture2D) -> Self {
        let w = image.width();
        let h = image.height();
        Self {
            image,
            w,
            h,
            speed: vec2(0., 0.),
            position: vec2(0., 0.),
            hitbox: Rect::new(0.0, 0.0, w, h),
        }
    }
    pub fn draw(&self) {
        //draw_texture(&self.image, 0., 0., WHITE);
        draw_texture(&self.image, self.position.x, self.position.y, WHITE);
    }

    pub fn set_in_window(&mut self, dt: f32) {
        self.position += self.speed * dt;

        let w = self.w;
        let h = self.h;

        self.position.x = self.position.x.clamp(0.0, screen_width() - w);
        self.position.y = self.position.y.clamp(0.0, screen_height() - h);

        //Mantener el hitbox sincronizado
        self.hitbox.x = self.position.x;
        self.hitbox.y = self.position.y;
        self.draw_hitbox();
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.w, self.h)
    }

    pub fn draw_hitbox(&self) {
        let hb = self.get_rect();
        //draw_rectangle_lines(self.hitbox.x, self.hitbox.y, self.hitbox.w, self.hitbox.h, 4.0, GREEN);
        draw_rectangle_lines(hb.x, hb.y, hb.w, hb.h, 4.0, GREEN);
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macroquad :: HitBox".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let dino_texture: Texture2D = load_texture("./dino.png").await.unwrap();
    dino_texture.set_filter(FilterMode::Nearest);
    let mut dino = Dino::new(dino_texture);
    
    loop {
        let dt = get_frame_time();

        clear_background(LIGHTGRAY); //BLACK

        dino.speed = vec2(0.0, 0.0);

        dino.draw();
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, BLACK);

        if is_key_down(KeyCode::Right) {
            //println!("KeyCode::Right");
            dino.speed.x = 100.0;
        }
        if is_key_down(KeyCode::Left) {
            //println!("KeyCode::Left");
            dino.speed.x = -100.0;
        }
        if is_key_down(KeyCode::Up) {
            //println!("KeyCode::Up");
            dino.speed.y = -100.0;
        }
        if is_key_down(KeyCode::Down) {
            //println!("KeyCode::Down");
            dino.speed.y = 100.0;
        }
        /*
        else {
            dino.speed.x = 0.;
            dino.speed.y = 0.0;
        }
        */

        dino.set_in_window(dt);

        draw_fps();

        window::next_frame().await;
    }
}

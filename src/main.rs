use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::*;
use macroquad::time::{draw_fps, get_frame_time};
use macroquad::window;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

#[allow(dead_code)]
const PHYSICS_DT: f32 = 1.0 / 120.0; //60

struct Dino {
    image: Texture2D,
    speed: Vec2,
    position: Vec2,
}

impl Dino {
    pub fn new(image: Texture2D) -> Self {
        Self {
            image,
            speed: vec2(0., 0.),
            position: vec2(0., 0.),
        }
    }
    pub fn draw(&self) {
        //draw_texture(&self.image, 0., 0., WHITE);
        draw_texture(&self.image, self.position.x, self.position.y, WHITE);
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

        dino.draw();
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, BLACK);

        if is_key_down(KeyCode::Right) {
            println!("KeyCode::Right");
            dino.speed.x = 100.0;
        } else if is_key_down(KeyCode::Left) {
            println!("KeyCode::Left");
            dino.speed.x = -100.0;
        } else if is_key_down(KeyCode::Up) {
            println!("KeyCode::Up");
            dino.speed.y = -100.0;
        } else if is_key_down(KeyCode::Down) {
            println!("KeyCode::Down");
            dino.speed.y = 100.0;
        } else {
            dino.speed.x = 0.;
            dino.speed.y = 0.0;
        }

        dino.position += dino.speed * dt;

        draw_fps();

        window::next_frame().await;
    }
}

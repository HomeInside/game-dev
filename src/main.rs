use macroquad::color::{BLACK, WHITE};
use macroquad::math::Rect;
use macroquad::prelude::*;
use macroquad::time::get_frame_time;
use macroquad::window;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

struct Player {
    image: Texture2D,
    w: f32,
    h: f32,
    position: Vec2,
    speed: Vec2,
    hitbox: Rect,
}

impl Player {
    pub fn new(image: Texture2D) -> Self {
        let w = image.width();
        let h = image.height();
        Self {
            image,
            w,
            h,
            position: vec2(0., 600.),
            speed: vec2(0., 0.),
            hitbox: Rect::new(0.0, 0.0, w, h),
        }
    }

    pub fn get_rect(&self) -> Rect {
        self.hitbox
    }

    pub fn draw_hitbox(&self) {
        let hb = self.get_rect();
        draw_rectangle_lines(hb.x, hb.y, hb.w, hb.h, 4.0, RED);
    }

    pub fn draw(&self) {
        draw_texture(&self.image, self.position.x, self.position.y, WHITE);
    }

    /// mantiene el player dentro de la
    // ventana principal
    pub fn set_in_window(&mut self, dt: f32) {
        self.position += self.speed * dt;

        let w = self.w;
        let h = self.h;

        self.position.x = self.position.x.clamp(0.0, screen_width() - w);
        self.position.y = self.position.y.clamp(0.0, screen_height() - h);

        // mantener el hitbox sincronizado
        self.hitbox.x = self.position.x;
        self.hitbox.y = self.position.y;
    }
}

struct Obstacle {
    rect: Rect,
    color: Color,
    hit_color: Color,
}

impl Obstacle {
    pub fn new(obst: Rect, color: Color, hit_color: Color) -> Self {
        Self {
            rect: obst,
            color,
            hit_color,
        }
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    pub fn draw_hitbox(&self, color: Color) {
        draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 4.0, color);
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.color);
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macroquad :: HitBox".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let player1: Texture2D = load_texture("./male_hero-design.png").await.unwrap();
    player1.set_filter(FilterMode::Nearest);
    let mut player1 = Player::new(player1);

    //x=hor, y=vert
    //x,y, ancho, alto
    let rect1 = Rect::new(320.0, 420.0, 100.0, 30.0);
    let obst1 = Obstacle::new(rect1, RED, GREEN);

    let rect2 = Rect::new(400.0, 500.0, 200.0, 30.0);
    let obst2 = Obstacle::new(rect2, GREEN, RED);

    let rect3 = Rect::new(150.0, 550.0, 300.0, 30.0);
    let obst3 = Obstacle::new(rect3, BLUE, RED);

    let mut platforms: Vec<Obstacle> = Vec::with_capacity(3);
    platforms.push(obst1);
    platforms.push(obst2);
    platforms.push(obst3);

    loop {
        let dt = get_frame_time();

        clear_background(WHITE); //BLACK

        player1.speed = vec2(0.0, 0.0);
        player1.draw();
        for platform in &platforms {
            platform.draw();
        }

        if is_key_down(KeyCode::Right) {
            player1.speed.x = 100.0;
        }
        if is_key_down(KeyCode::Left) {
            player1.speed.x = -100.0;
        }
        if is_key_down(KeyCode::Up) {
            player1.speed.y = -100.0;
        }
        if is_key_down(KeyCode::Down) {
            player1.speed.y = 100.0;
        }

        if is_key_down(KeyCode::Space) {
            println!("KeyCode::Space");
            println!("jump!");
        }

        for platform in &platforms {
            if player1.get_rect().overlaps(&platform.get_rect()) {
                player1.draw_hitbox();
                platform.draw_hitbox(platform.hit_color);
            }
        }

        player1.set_in_window(dt);

        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., BLACK);

        window::next_frame().await;
    }
}

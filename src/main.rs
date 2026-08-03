use macroquad::color::{BLACK, WHITE};
use macroquad::math::Rect;
use macroquad::prelude::*;
use macroquad::time::get_frame_time;
use macroquad::window;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
//Después lo ajustaremos.
const GRAVITY: f32 = 600.0;

struct Player {
    image: Texture2D,
    w: f32,
    h: f32,
    position: Vec2,
    speed: Vec2,
    hitbox: Rect,
    is_grounded: bool,
}

impl Player {
    pub fn new(image: Texture2D) -> Self {
        let w = image.width();
        let h = image.height();

        // el jugador empienza en la parte
        // inferior izquieda, sobre el "piso/suelo"

        Self {
            image,
            w,
            h,
            position: vec2(0., 600.),
            speed: vec2(0., 0.),
            hitbox: Rect::new(0.0, 0.0, w, h),
            is_grounded: true,
        }
    }

    /// obtener el hitbox
    pub fn get_rect(&self) -> Rect {
        self.hitbox
    }

    /// mantener el hitbox sincronizado
    pub fn sync_hitbox(&mut self) {
        self.hitbox.x = self.position.x;
        self.hitbox.y = self.position.y;
    }

    /// dibuja el hitbox, el rect que
    /// delimita el tamaño del jugador.
    pub fn draw_hitbox(&self) {
        let hb = self.get_rect();
        draw_rectangle_lines(hb.x, hb.y, hb.w, hb.h, 4.0, RED);
    }

    /// dibuja al jugador
    pub fn draw(&self) {
        draw_texture(&self.image, self.position.x, self.position.y, WHITE);
    }

    /// DEPRECATED
    /// mantiene el player dentro de la
    // ventana principal
    pub fn set_in_window(&mut self, _dt: f32) {
        //ahora es `update_position()`
        //self.position += self.speed * dt;

        //horizontal/paredes
        let w = self.w;
        // vertical/suelo
        //let h = self.h;

        //horizontal/paredes
        self.position.x = self.position.x.clamp(0.0, screen_width() - w);
        // vertical/suelo
        //self.position.y = self.position.y.clamp(0.0, screen_height() - h);

        //ahora es `update_position()`
        // mantener el hitbox sincronizado
        //self.hitbox.x = self.position.x;
        //self.hitbox.y = self.position.y;
    }

    /// modifica la velocidad aplicando gravedad.
    pub fn apply_gravity(&mut self, dt: f32) {
        self.speed.y += GRAVITY * dt;
    }

    /// mueve al jugador.
    pub fn update(&mut self, dt: f32) {
        self.position += self.speed * dt;
    }

    /// mueve al jugador.
    pub fn update_position(&mut self, dt: f32) {
        self.position += self.speed * dt;

        // mantener el hitbox sincronizado
        self.sync_hitbox();
    }

    /// el piso será `screen_height()`, una línea
    /// horizontal invisible, que delimita la
    /// ventana del juego.
    /// valida que el jugador no caiga/salga más
    /// allá del piso, corrigiendo la posición y
    /// velocidad en caso de que lo haga.
    pub fn resolve_floor(&mut self) {
        let floor_y = screen_height();

        self.is_grounded = false;

        if self.position.y + self.h >= floor_y {
            self.position.y = floor_y - self.h;
            self.speed.y = 0.0;
            self.is_grounded = true;
        }

        self.sync_hitbox();
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

        player1.apply_gravity(dt);
        player1.update_position(dt);
        player1.resolve_floor();

        player1.set_in_window(dt);
        //player1.update(dt);

        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 24., BLACK);
        draw_text(
            format!("toca el suelo: {}", player1.is_grounded).as_str(),
            0.,
            35.,
            24.,
            BLACK,
        );

        window::next_frame().await;
    }
}

use macroquad::color::{BLACK, WHITE};
use macroquad::math::Rect;
use macroquad::prelude::*;
use macroquad::time::get_frame_time;
use macroquad::window;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

// Después lo ajustaremos.
const GRAVITY: f32 = 600.0;
const JUMP_SPEED: f32 = -350.0; //-350.0;

struct Player {
    image: Texture2D,
    w: f32,
    h: f32,
    position: Vec2,
    speed: Vec2,
    hitbox: Rect,
    is_grounded: bool,
    previous_position: Vec2,
}

impl Player {
    pub fn new(image: Texture2D) -> Self {
        let w = image.width();
        let h = image.height();

        // el jugador empieza en la parte inferior izquierda
        // sobre el "piso/suelo", recuerda que el jugador tiene
        // altura, asi que debe empezar "encima" del "piso/suelo"
        // sin embargo `resolve_floor` corrige la posición,
        // un bug corregido de forma inesperada :)
        //screen_height() - player.h // 560
        let initial_post = screen_height() - h;

        Self {
            image,
            w,
            h,
            position: vec2(0., initial_post),
            speed: vec2(0., 0.),
            hitbox: Rect::new(0.0, 0.0, w, h),
            is_grounded: true,
            previous_position: vec2(0., initial_post),
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

    // TODO
    // mueve el eje X, del jugador
    pub fn move_x(&mut self, dt: f32) {
        self.position.x += self.speed.x * dt;
        self.sync_hitbox();
    }

    // TODO
    // mueve el eje Y, del jugador
    pub fn move_y(&mut self, dt: f32) {
        self.position.y += self.speed.y * dt;
        self.sync_hitbox();
    }

    /// mueve al jugador.
    pub fn update(&mut self, dt: f32) {
        self.position += self.speed * dt;
    }

    /// DEPRECATED por move_x y move_y
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

        if self.position.y + self.h >= floor_y {
            self.position.y = floor_y - self.h;
            self.speed.y = 0.0;
            self.is_grounded = true;
        }

        self.sync_hitbox();
    }

    /// salto
    pub fn jump(&mut self) {
        println!("jump fn");
        if self.is_grounded {
            println!("player jump!");
            self.speed.y = JUMP_SPEED;
            self.is_grounded = false;
        }
    }

    /// DEPRECATED ver `resolve_platform_x`
    pub fn resolve_platform(&mut self, platform: Rect) {
        if self.hitbox.overlaps(&platform) {
            if self.speed.y > 0.0 {
                self.position.y = platform.y - self.h;
                self.speed.y = 0.0;
                self.is_grounded = true;
            }

            self.sync_hitbox();
        }
    }

    /// colisiones horizonatales.
    pub fn resolve_platform_x(&mut self, platform: Rect) {
        if !self.hitbox.overlaps(&platform) {
            return;
        }

        // Venía desde la izquierda
        if self.speed.x > 0.0 && self.previous_position.x + self.w <= platform.x {
            self.position.x = platform.x - self.w;
            self.speed.x = 0.0;
        }
        // Venía desde la derecha
        else if self.speed.x < 0.0 && self.previous_position.x >= platform.x + platform.w {
            self.position.x = platform.x + platform.w;
            self.speed.x = 0.0;
        }

        self.sync_hitbox();
    }

    /// colisiones verticales.
    pub fn resolve_platform_y(&mut self, platform: Rect) {
        if !self.hitbox.overlaps(&platform) {
            return;
        }

        // Cayendo: aterrizar encima
        if self.speed.y > 0.0 && self.previous_position.y + self.h <= platform.y {
            self.position.y = platform.y - self.h;
            self.speed.y = 0.0;
            self.is_grounded = true;
        }
        // Subiendo: golpear techo
        else if self.speed.y < 0.0 && self.previous_position.y >= platform.y + platform.h {
            self.position.y = platform.y + platform.h;
            self.speed.y = 0.0;
        }

        self.sync_hitbox();
    }

    /// guardar posicion anterior
    pub fn save_previous_position(&mut self) {
        self.previous_position = self.position;
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

    pub fn draw_hitbox(&self) {
        draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 4.0, self.hit_color);
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

        // 1. INPUT
        let mut direction = 0.0;

        if is_key_down(KeyCode::Right) {
            direction += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            direction -= 1.0;
        }

        //debug keys
        if is_key_down(KeyCode::A) {
            player1.position.x = 0.0;
            player1.position.y = 560.0;
            player1.sync_hitbox();
        }

        //debug keys
        if is_key_down(KeyCode::S) {
            player1.position.x = 760.0;
            player1.position.y = 560.0;
            player1.sync_hitbox();
        }

        if is_key_pressed(KeyCode::Space) {
            println!("grounded antes del salto: {}", player1.is_grounded);
            player1.jump();
        }

        player1.speed.x = direction * 100.0;

        // 2. PHYSICS
        player1.apply_gravity(dt);

        // guardar la posición actual
        player1.save_previous_position();

        // 3. MOVEMENT
        // player1.update_position(dt);
        player1.move_x(dt);

        // 4. COLLISIONS
        // valida si el jugador choca con una plataforma
        for platform in &platforms {
            player1.resolve_platform_x(platform.get_rect());
        }

        player1.move_y(dt);

        player1.is_grounded = false;

        for platform in &platforms {
            player1.resolve_platform_y(platform.get_rect());
        }

        player1.resolve_floor();

        player1.set_in_window(dt);

        //5. DRAW
        player1.draw();
        for platform in &platforms {
            platform.draw();
            if player1.get_rect().overlaps(&platform.get_rect()) {
                player1.draw_hitbox();
                platform.draw_hitbox();
            }
        }

        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 24., BLACK);
        draw_text(
            format!("toca el suelo: {}", player1.is_grounded).as_str(),
            0.,
            35.,
            24.,
            BLACK,
        );
        draw_text(
            format!("Player X: {:.1}, y: {:.1}", player1.position.x, player1.position.y).as_str(),
            0.,
            55.,
            24.,
            BLACK,
        );

        window::next_frame().await;
    }
}

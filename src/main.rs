// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, GraphicsContext, Image, Mesh, Rect};
use ggez::input::keyboard::KeyInput;
use ggez::{Context, GameResult, glam};
use glam::Vec2;
use winit::keyboard::{Key, NamedKey};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

struct Dino {
    image: Image,
    w: f32,
    h: f32,
    position: Vec2,
    speed: Vec2,
    hitbox: Rect,
}

impl Dino {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let image = Image::from_path(ctx, "/dino.png")?;
        let w = image.width() as f32;
        let h = image.height() as f32;

        Ok(Self {
            image,
            w,
            h,
            position: Vec2::new(0., 0.),
            speed: Vec2::new(0., 0.),
            hitbox: Rect::new(0.0, 0.0, w, h),
        })
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.w, self.h)
    }

    pub fn draw_hitbox(&self, canvas: &mut Canvas, gfx: &mut GraphicsContext) {
        let hb = self.get_rect();
        let hitbox_mesh = Mesh::new_rectangle(gfx, DrawMode::stroke(2.0), hb, Color::RED)
            .expect("Error al crear rectángulo de debug");

        canvas.draw(&hitbox_mesh, DrawParam::default());
    }

    pub fn update(&mut self, dt: f32, screen_size: glam::Vec2) {
        self.position += self.speed * dt;

        self.set_in_window(screen_size.x, screen_size.y);
    }

    fn draw(&self, canvas: &mut graphics::Canvas) -> GameResult {
        let params = DrawParam::default().dest(Vec2::new(self.position.x, self.position.y));

        canvas.draw(&self.image, params);
        Ok(())
    }

    /// mantiene el dinosaurio dentro de la
    // ventana principal
    pub fn set_in_window(&mut self, x: f32, y: f32) {
        let w = self.w;
        let h = self.h;

        self.position.x = self.position.x.clamp(0.0, x - w);
        self.position.y = self.position.y.clamp(0.0, y - h);

        // mantener el hitbox sincronizado
        self.hitbox.x = self.position.x;
        self.hitbox.y = self.position.y;
    }
}

struct Obstacle {
    rect: Rect,
    color: Color,
}

impl Obstacle {
    pub fn new(obst: Rect, color: Color) -> Self {
        Self { rect: obst, color }
    }

    pub fn get_rect(&self) -> Rect {
        self.rect
    }

    pub fn draw_hitbox(&self, canvas: &mut Canvas, gfx: &mut GraphicsContext) {
        let hb = self.get_rect();
        let hitbox_mesh = Mesh::new_rectangle(gfx, DrawMode::stroke(2.0), hb, Color::GREEN)
            .expect("Error al crear rectángulo de debug");

        canvas.draw(&hitbox_mesh, DrawParam::default());
    }

    fn draw(&self, canvas: &mut graphics::Canvas, gfx: &mut GraphicsContext) -> GameResult {
        let mesh =
            Mesh::new_rectangle(gfx, DrawMode::fill(), self.rect, self.color).expect("Error al crear el rectángulo");

        let params = DrawParam::default();
        canvas.draw(&mesh, params);
        Ok(())
    }
}

fn draw_fps(ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
    let fps_counter = ctx.time.fps();

    let fps_text = format!("FPS: {:.0}", fps_counter);
    let mut fps_text_layout = graphics::Text::new(fps_text);
    fps_text_layout.set_scale(graphics::PxScale::from(22.0));

    // score en la parte superior izquierda
    let fps_text_position = Vec2::new(0.0, 1.0);
    canvas.draw(
        &fps_text_layout,
        DrawParam::default()
            .dest(fps_text_position)
            .color(graphics::Color::BLACK),
    );
    Ok(())
}

fn draw_collide_info(canvas: &mut graphics::Canvas) -> GameResult {
    let info_text = format!("objetos colisionando!");
    let mut info_text_layout = graphics::Text::new(info_text);
    info_text_layout.set_scale(graphics::PxScale::from(22.0));

    // en la parte superior izquierda
    let info_text_position = Vec2::new(0.0, 20.0);
    canvas.draw(
        &info_text_layout,
        DrawParam::default()
            .dest(info_text_position)
            .color(graphics::Color::RED),
    );
    Ok(())
}

// el struct principal
struct MainState {
    dino: Dino,
    obstacle: Obstacle,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let dino = Dino::new(ctx)?;
        let screen_size = ctx.gfx.drawable_size();
        let screen_width: glam::Vec2 = screen_size.into();

        let rectangle = Rect::new(screen_width.y / 2.0 + 50.0, 150.0, 220.0, 120.0);
        let obstacle = Obstacle::new(rectangle, Color::BLACK);

        Ok(MainState { dino, obstacle })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        let screen_size = ctx.gfx.drawable_size();

        self.dino.update(dt, screen_size.into());

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        self.dino.speed = Vec2::new(0., 0.);
        self.dino.draw(&mut canvas)?;
        self.obstacle.draw(&mut canvas, &mut ctx.gfx)?;

        let is_colliding = self.dino.get_rect().overlaps(&self.obstacle.get_rect());

        if is_colliding {
            self.dino.draw_hitbox(&mut canvas, &mut ctx.gfx);
            self.obstacle.draw_hitbox(&mut canvas, &mut ctx.gfx);
            draw_collide_info(&mut canvas)?;
        }

        draw_fps(ctx, &mut canvas)?;

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        if input.event.logical_key == Key::Named(NamedKey::ArrowRight) {
            //println!("KeyCode::Right");
            self.dino.speed.x = 100.0;
        }
        if input.event.logical_key == Key::Named(NamedKey::ArrowLeft) {
            //println!("KeyCode::Left");
            self.dino.speed.x = -100.0;
        }
        if input.event.logical_key == Key::Named(NamedKey::ArrowUp) {
            //println!("KeyCode::Up");
            self.dino.speed.y = -100.0;
        }
        if input.event.logical_key == Key::Named(NamedKey::ArrowDown) {
            //println!("KeyCode::Down");
            self.dino.speed.y = 100.0;
        }
        /*
        match input.event.logical_key {
            Key::Named(NamedKey::ArrowRight) => {
                //println!("KeyCode::Right");
                self.dino.speed.x = 100.0;
            }
            Key::Named(NamedKey::ArrowLeft) => {
                //println!("KeyCode::Left");
                self.dino.speed.x = -100.0;
            }
            Key::Named(NamedKey::ArrowUp) => {
                //println!("KeyCode::Up");
                self.dino.speed.y = -100.0;
            }
            Key::Named(NamedKey::ArrowDown) => {
                //println!("KeyCode::Down");
                self.dino.speed.y = 100.0;
            }
            Key::Named(NamedKey::Escape) => {
                println!("saliendo...");
                //ctx.request_quit();
            }
            _ => {}
        }
        */
        Ok(())
    }
}

fn main() -> GameResult {
    let resource_dir = std::path::PathBuf::from("./");

    let cb = ggez::ContextBuilder::new("HitBox", "Helio Studio Games")
        .window_setup(conf::WindowSetup::default().title("ggez :: HitBox"))
        .window_mode(
            conf::WindowMode::default()
                .dimensions(WIDTH, HEIGHT)
                .maximized(false)
                .resizable(false),
        )
        .add_resource_path(&resource_dir);

    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}

// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

// puede ser src/main_menu.rs
struct MainMenu;

impl MainMenu {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(unused_variables)]
    pub fn update(&mut self, dt: f32) {
        // lógica de MainMenu
    }

    pub fn draw(&self) {
        clear_background(GREEN);
        let text = "MAIN MENU";
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            screen_width() / 2.0 - text_dimensions.width / 2.0,
            screen_height() / 2.0,
            50.0,
            RED,
        );
    }
}

// puede ser src/playing.rs
struct Playing;

impl Playing {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(unused_variables)]
    pub fn update(&mut self, dt: f32) {
        // lógica de Playing
    }

    pub fn draw(&self) {
        clear_background(YELLOW);
        let text = "PLAYING";
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            screen_width() / 2.0 - text_dimensions.width / 2.0,
            screen_height() / 2.0,
            50.0,
            RED,
        );
    }
}

// puede ser src/paused.rs
struct Paused;

impl Paused {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(unused_variables)]
    pub fn update(&mut self, dt: f32) {
        // lógica de Paused
    }

    pub fn draw(&self) {
        clear_background(BLUE);
        let text = "PAUSED";
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            screen_width() / 2.0 - text_dimensions.width / 2.0,
            screen_height() / 2.0,
            50.0,
            RED,
        );
    }
}

// puede ser src/game_over.rs
struct GameOver;

impl GameOver {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(unused_variables)]
    pub fn update(&mut self, dt: f32) {
        // lógica de GameOver
    }

    pub fn draw(&self) {
        clear_background(BLACK);
        let text = "GAME OVER";
        let text_dimensions = measure_text(text, None, 50, 1.0);
        draw_text(
            text,
            screen_width() / 2.0 - text_dimensions.width / 2.0,
            screen_height() / 2.0,
            50.0,
            RED,
        );
    }
}

// cada variante contiene un struct
// que se encarga de implementar el
// comportamiento específico de cada
// estado.
// Esto garantiza que el juego se encuentra
// en un único estado a la vez.
enum GameState {
    MainMenu(MainMenu),
    Playing(Playing),
    Paused(Paused),
    GameOver(GameOver),
}

// la implementación agrega operaciones
// comunes a todos los estados.
impl GameState {
    pub fn update(&mut self, dt: f32) {
        match self {
            Self::MainMenu(scene) => {
                scene.update(dt);
            }

            Self::Playing(scene) => {
                scene.update(dt);
            }

            Self::Paused(scene) => {
                scene.update(dt);
            }

            Self::GameOver(scene) => {
                scene.update(dt);
            }
        }
    }

    pub fn draw(&self) {
        match self {
            Self::MainMenu(scene) => {
                scene.draw();
            }

            Self::Playing(scene) => {
                scene.draw();
            }

            Self::Paused(scene) => {
                scene.draw();
            }

            Self::GameOver(scene) => {
                scene.draw();
            }
        }
    }
}

// puede ser src/game.rs
struct Game {
    current_state: GameState,
    bg_color: Color,
}

// implementacion principal del juego
impl Game {
    pub fn new() -> Self {
        Self {
            current_state: GameState::MainMenu(MainMenu::new()),
            bg_color: BLACK,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // cambiar de estado

        if is_key_pressed(KeyCode::Key1) {
            self.current_state = GameState::MainMenu(MainMenu::new());
            self.bg_color = BLACK;
        } else if is_key_pressed(KeyCode::Key2) {
            self.current_state = GameState::Playing(Playing::new());
            self.bg_color = BLACK;
        } else if is_key_pressed(KeyCode::Key3) {
            self.current_state = GameState::Paused(Paused::new());
            self.bg_color = BLACK;
        } else if is_key_pressed(KeyCode::Key4) {
            self.current_state = GameState::GameOver(GameOver::new());
            self.bg_color = WHITE;
        }

        // actualizar la escena activa
        self.current_state.update(dt);
    }

    pub fn draw(&self) {
        // dibujar la escena activa
        self.current_state.draw();
        self.draw_hud();
    }

    fn draw_hud(&self) {
        // mostrar controles
        let controls = "1: Main Menu | 2: Playing | 3: Paused | 4: GameOver | Q: Salir";

        let ctrl_size = measure_text(controls, None, 18, 1.0);

        draw_text(
            controls,
            screen_width() - ctrl_size.width - 20.0,
            screen_height() - 20.0,
            18.0,
            self.bg_color,
        );

        // dibujar FPS
        let fps = format!("FPS: {}", (1.0 / get_frame_time()) as i32);

        draw_text(&fps, 20.0, screen_height() - 20.0, 18.0, self.bg_color);
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macro::state".to_owned(),
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
    let mut game = Game::new();

    loop {
        clear_background(WHITE);

        let dt = get_frame_time();

        if is_key_pressed(KeyCode::Q) {
            println!("saliendo...");
            break;
        }

        game.update(dt);
        game.draw();

        next_frame().await;
    }

    Ok(())
}

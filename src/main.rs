#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

mod rain;
mod snow;
mod summer;
use rain::Rain;
use snow::Snow;
use summer::Summer;

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macro::seasons".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), macroquad::Error> {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // color de fondo
    let back_color = Color::new(20.0 / 255.0, 25.0 / 255.0, 40.0 / 255.0, 1.0);

    // color de la lluvia
    // mas gris
    let rain_color = Color::new(224.0 / 255.0, 231.0 / 255.0, 246.0 / 255.0, 150.0 / 255.0);

    // crear la lluvia
    let mut rain = Rain::new(screen_width, screen_height, 600, rain_color);

    // crear la nieve, densa pero suave
    let mut snow = Snow::new(screen_width, screen_height, 800);

    // crear el verano
    let mut summer = Summer::new(screen_width, screen_height).await;

    let mut set_seasion = 1;

    let winter_img = load_texture("winter-is-coming.png").await.unwrap();
    let vegeta_img = load_texture("vegeta-lluvia.png").await.unwrap();

    loop {
        let dt = get_frame_time();

        if is_key_down(KeyCode::Key1) {
            set_seasion = 1;
        } else if is_key_down(KeyCode::Key2) {
            set_seasion = 2;
        } else if is_key_down(KeyCode::Key3) {
            set_seasion = 3;
        }

        if set_seasion == 1 {
            clear_background(back_color);
            rain.update(dt);
            draw_texture_ex(
                &vegeta_img,
                (screen_width / 2.0) - 720.0,
                (screen_height / 2.0) - 250.0, //450
                WHITE,
                DrawTextureParams {
                    flip_x: true,
                    ..Default::default()
                },
            );
            rain.draw();
        } else if set_seasion == 2 {
            clear_background(WHITE);
            summer.update(dt);
            summer.draw(dt);
        } else if set_seasion == 3 {
            clear_background(back_color);
            snow.update(dt);
            draw_texture(
                &winter_img,
                (screen_width / 2.0) + 120.0,
                (screen_height / 2.0) - 50.0,
                WHITE,
            );
            snow.draw();
        }

        draw_text(format!("FPS: {}", get_fps()).as_str(), 0.0, 18.0, 32.0, LIGHTGRAY);
        draw_text(
            "Lluvia [1], Verano [2], Nieve [3]",
            screen_width / 2.0 - 250.0,
            18.0,
            32.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}

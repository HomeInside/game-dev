#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macro::seasons".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        high_dpi: true,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), macroquad::Error> {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // color de fondo
    let back_color = Color::new(20.0 / 255.0, 25.0 / 255.0, 40.0 / 255.0, 1.0);

    loop {
        let dt = get_frame_time();

        clear_background(back_color);

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

    Ok(())
}

// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macro::text".to_owned(),
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
    let message = "This sample illustrates a text writing\nanimation effect! Check it out! ;)";
    let text_dim = measure_text(message, None, 30, 1.0);

    let mut frames_counter = 0;

    loop {
        clear_background(WHITE);

        let dt = get_frame_time();

        if is_key_down(KeyCode::Space) {
            frames_counter += 8;
        } else {
            frames_counter += 1;
        }

        if is_key_down(KeyCode::Enter) {
            frames_counter = 0;
        }

        if is_key_pressed(KeyCode::Q) {
            println!("saliendo...");
            break;
        }

        /*
        draw_text(
            message,
            screen_width() / 2.0 - text_dim.width / 2.0,
            screen_height() / 2.0,
            40.0,
            RED,
        );
        */

        //
        // c++
        // DrawText(TextSubtext(message, 0, framesCounter/10), 210, 160, 20, MAROON);

        let cantidad = (frames_counter / 10) as usize;

        let texto: String = message.chars().take(cantidad).collect();

        draw_text(
            &texto,
            //210.0, 160.0,
            //screen_width() / 2.0 - text_dim.width / 2.0,
            10.0,
            screen_height() / 2.0,
            //
            30.0,
            RED,
        );

        next_frame().await;
    }

    Ok(())
}

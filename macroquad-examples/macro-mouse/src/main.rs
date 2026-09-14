// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macro::mouse".to_owned(),
        window_width: 900,
        window_height: 602,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), macroquad::Error> {
    let alpha = Color::new(255.0, 0.0, 0.0, 255.0);

    let fondo = load_texture("background_00.png").await.unwrap();
    let mira = load_texture("center.png").await.unwrap();

    let mut has_click: bool = false;
    let mut time: f32 = 0.0;
    let mut saved_x: f32 = 0.0;
    let mut saved_y: f32 = 0.0;

    show_mouse(false);

    loop {
        clear_background(WHITE);

        let dt = get_frame_time();
        time += dt;

        draw_texture_ex(
            &fondo,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        let (x, y) = mouse_position();

        // is_mouse_button_down -> true, mientras mantienes pulsado el botón
        // is_mouse_button_released -> true, durante el frame en que sueltas el botón
        if is_mouse_button_pressed(MouseButton::Left) {
            println!("click izquierdo: X:{}, Y:{}", x, y);
            // guardar las coordenadas donde se hizo el click
            saved_x = x;
            saved_y = y;
            // fijar a true, para activar el mensaje
            has_click = true;
            // resetear el tiempo
            time = 0.0;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            println!("click derecho: X:{}, Y:{}", x, y);
        }

        if is_key_down(KeyCode::M) {
            // dibujar la `mira` centrada en el cursor
            draw_texture(&mira, x - mira.width() / 2.0, y - mira.height() / 2.0, WHITE);
        }

        if is_key_down(KeyCode::N) {
            // mira con circulos y lineas
            //draw_circle(x, y, 20.0, alpha);
            // dibuja el contorno del círculo
            draw_circle_lines(x, y, 15.0, 2.0, RED);

            draw_line(x - 25.0, y, x - 5.0, y, 2.0, RED); // linea izda
            draw_line(x + 5.0, y, x + 25.0, y, 2.0, RED); // linea derecha

            draw_line(x, y - 25.0, x, y - 5.0, 2.0, RED); // linea superior
            draw_line(x, y + 5.0, x, y + 25.0, 2.0, RED); // linea inferior
        }

        //
        draw_text(format!("FPS: {}", get_fps()).as_str(), 2.0, 15.0, 24.0, BLACK);
        draw_text(format!("mouse: X:{}, Y:{}", x, y).as_str(), 2.0, 25.0, 24.0, BLACK);

        if has_click {
            time += dt;

            // mientras el tiempo sea menor que 2 segundos
            // mostrar el mensaje
            if time < 2.0 {
                draw_text(
                    format!("click en: X:{}, Y:{}", saved_x, saved_y).as_str(),
                    2.0,
                    45.0,
                    32.0,
                    RED,
                );
            } else {
                // cuando llegue a 2 segundos
                // desactivar el mensaje
                has_click = false;
            }
        }

        next_frame().await;
    }
}

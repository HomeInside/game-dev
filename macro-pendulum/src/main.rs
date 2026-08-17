// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macro::pendulum".to_owned(),
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
    let length = 200.0;
    let g = 981.0;

    let mut theta: f32 = 1.5; //1.0|1.5;
    let mut omega: f32 = 0.0;

    loop {
        let dt = get_frame_time();

        // Física
        // sin amortiguamiento (pendulo ideal)
        // let alpha = -(g / length) * theta.sin();
        //
        // con amortiguamiento (se va deteniendo progresivamente)
        // al incrementar el valor se siente "mas pesado", aunque
        // aqui no se usa masa
        let damping = 0.2; //0.5|1.0

        let alpha = -(g / length) * theta.sin() - damping * omega;

        omega += alpha * dt;
        theta += omega * dt;

        // posición del pivote
        let pivot_x = screen_width() / 2.0;
        let pivot_y = 150.0;

        let bob_x = pivot_x + length * theta.sin();
        let bob_y = pivot_y + length * theta.cos();

        clear_background(WHITE);

        draw_line(pivot_x, pivot_y, bob_x, bob_y, 4.0, BLACK);

        draw_circle(pivot_x, pivot_y, 8.0, DARKGRAY);

        draw_circle(bob_x, bob_y, 20.0, RED);

        // matemáticamente el movimiento se acerca a cero
        // y acercarse a cero no es lo mismo que llegar
        // exactamente a cero
        // además, el amortiguamiento es proporcional a omega
        // cuando omega se hace muy pequeña, la fuerza de
        // amortiguamiento también se hace muy pequeña
        // para este caso: si el movimiento es suficientemente pequeño,
        // se considera que está quieto cuando se obtiene los
        // siguientes valores
        //
        if omega.abs() < 0.001 && theta.abs() < 0.001 {
            omega = 0.0;
            theta = 0.0;
        }

        draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 29.0, 24.0, BLACK);
        draw_text(format!("theta: {}", theta).as_str(), 10.0, 49.0, 24.0, BLACK);
        draw_text(format!("omega: {}", omega).as_str(), 10.0, 65.0, 24.0, BLACK);

        next_frame().await;
    }
}

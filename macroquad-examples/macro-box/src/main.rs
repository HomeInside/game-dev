// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macro::box-patrol".to_owned(),
        window_width: 900,
        window_height: 600,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

/// Esta función calcula dónde debe estar Box1 en cada momento:
/// recibe el centro del Box2, su tamaño, la distancia recorrida y el tamaño del Box1
/// calcula el perímetro total `(2 * ancho + 2 * alto)`
/// usa el módulo (%) para que la distancia siempre esté dentro del perímetro
/// divide el perímetro en 4 segmentos y calcula la posición según en qué lado esté
/// y aplica un "offset(desplazamiento)" para que Box1 quede pegado al borde
/// exterior, no encima de Box2.
///
fn perimeter_position(center: Vec2, size: Vec2, distance: f32, b1_size: f32) -> Vec2 {
    let half = size / 2.0;
    let offset = b1_size / 2.0; // Para que Box1 esté pegado al borde exterior

    let top_side = size.x;
    let right_side = size.y;
    let bottom_side = size.x;
    let left_side = size.y;
    let total = 2.0 * (size.x + size.y);

    let mut d = distance % total;

    if d < top_side {
        // lado superior: izquierda a derecha
        let x = -half.x + d;
        Vec2::new(center.x + x, center.y - half.y - offset)
    } else if d < top_side + right_side {
        // lado derecho: arriba a abajo
        let y = d - top_side;
        Vec2::new(center.x + half.x + offset, center.y - half.y + y)
    } else if d < top_side + right_side + bottom_side {
        // lado inferior: derecha a izquierda
        let x = d - top_side - right_side;
        Vec2::new(center.x + half.x - x, center.y + half.y + offset)
    } else {
        // lado izquierdo: abajo a arriba
        let y = d - top_side - right_side - bottom_side;
        Vec2::new(center.x - half.x - offset, center.y + half.y - y)
    }
}

/// El cuadrado azul (Box1) recorre el perímetro exterior
/// de un rectángulo rojo (Box2) de forma contínua, como si estuviera
/// "orbitando" alrededor de él.
/// Box1: Cuadrado azul de 30x30 píxeles (el que se mueve)
/// Box2: Rectángulo rojo de 200x150 píxeles (estático, en el centro)
/// Velocidad: 100 píxeles por segundo.
///
#[macroquad::main(window_conf)]
async fn main() -> Result<(), macroquad::Error> {
    //tamaño del Box 1
    let box1_size = 30.0;
    //tamaño del Box 2
    let box2_size = vec2(200.0, 150.0);
    let box2_pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let speed = 100.0; // en píxeles/seg

    // Estado del recorrido
    let half = box2_size / 2.0;
    let perimeter = 2.0 * (box2_size.x + box2_size.y);
    let mut distance = 0.0; // distancia recorrida sobre el perímetro

    loop {
        clear_background(WHITE);
        let dt = get_frame_time();

        distance += speed * dt;

        if distance >= perimeter {
            distance -= perimeter;
        }

        // la posición de Box1 en el perímetro
        let b1_pos = perimeter_position(box2_pos, box2_size, distance, box1_size);

        // Dibujar Box1 ( rect externo, pegado al borde exterior)
        draw_rectangle(
            b1_pos.x - box1_size / 2.0,
            b1_pos.y - box1_size / 2.0,
            box1_size,
            box1_size,
            BLUE,
        );

        // Dibujar Box2 (rect interno)
        draw_rectangle(
            box2_pos.x - box2_size.x / 2.0,
            box2_pos.y - box2_size.y / 2.0,
            box2_size.x,
            box2_size.y,
            RED,
        );

        next_frame().await;
    }
}

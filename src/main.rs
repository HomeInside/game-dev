//use macroquad::prelude::*;
use macroquad::color::{BLACK, WHITE};
//use macroquad::color::{Color, colors::LIGHTGRAY};
use macroquad::math::{Rect, vec2};
//use macroquad::window;
//use macroquad::shapes::draw_circle;
use macroquad::input::{KeyCode, is_key_down};
use macroquad::text::draw_text;
use macroquad::texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture};
//use macroquad::time::{draw_fps, get_fps};
use macroquad::time::get_frame_time;
use macroquad::window::{clear_background, next_frame};

#[macroquad::main("Animation Test")]
async fn main() {
    // 1. cargar la textura(el Sprite Sheet)
    let texture: Texture2D = load_texture("assets/male_hero-run.png").await.unwrap();

    // 2. calcular datos básicos
    let sprite_size = 128.0; // todos miden 128x128, falta revisar
    let total_frames = (texture.width() / sprite_size) as usize; // 1280 / 128 = 10
    let frame_speed = 0.1; // cambia el cuadro ó frame cada 0.1 segundos

    let mut current_frame = 0;
    let mut timer = 0.0;
    let mut position = vec2(300.0, 250.0);
    let mut facing_right = true; // si el jugador mira a la derecha o izda

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        // 3. Manejo de Input
        let mut speed = 0.0;

        if is_key_down(KeyCode::Right) {
            speed = 200.0;
            facing_right = true;
        }
        if is_key_down(KeyCode::Left) {
            speed = -200.0;
            facing_right = false;
        }

        position.x += speed * dt;

        // 4. Lógica de la animación (cambiar el frame)
        timer += dt;
        if timer >= frame_speed {
            timer = 0.0;
            current_frame += 1;
            if current_frame >= total_frames {
                current_frame = 0; // Loop infinito
            }
        }

        // 5. Cálculo del área de recorte de la textura
        // aqui se obtiene el frame del Sprite Sheet
        let source_x = current_frame as f32 * sprite_size;
        let source_rect = Rect::new(source_x, 0.0, sprite_size, texture.height());

        // 6. Dibujar la textura recortada (el frame, usando DrawTextureParams)
        draw_texture_ex(
            &texture,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                flip_x: !facing_right, // gira el frame si va a la izquierda
                ..Default::default()
            },
        );

        draw_text("Usa FLECHA IZQUIERDA y DERECHA para moverte", 0.0, 20.0, 25.0, WHITE);
        draw_text(&format!("Frame actual: {}", current_frame), 0.0, 50.0, 25.0, WHITE);

        next_frame().await;
    }
}

//use macroquad::prelude::*;
use macroquad::color::{BLACK, WHITE};
//use macroquad::color::{Color, colors::LIGHTGRAY};
use macroquad::math::{Rect, vec2};
//use macroquad::window;
//use macroquad::shapes::draw_circle;
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::text::draw_text;
use macroquad::texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture};
//use macroquad::time::{draw_fps, get_fps};
use macroquad::time::get_frame_time;
use macroquad::window::{self, clear_background, next_frame, screen_height};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const GRAVITY: f32 = 600.0; // La fuerza de gravedad
const JUMP_SPEED: f32 = -350.0; // velocidad inicial del salto(Empuje inicial hacia arriba

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macroquad :: sprite-anim".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // 1. cargar la textura(el Sprite Sheet)
    // para este ejemplo todos los Sprite Sheet
    // tienen el mismo tamaño y la misma cantidad de frames
    let texture_walk: Texture2D = load_texture("assets/male_hero-walk.png").await.unwrap();
    let texture_run: Texture2D = load_texture("assets/male_hero-run.png").await.unwrap();
    let texture_idle: Texture2D = load_texture("assets/male_hero-idle.png").await.unwrap();
    let texture_jump: Texture2D = load_texture("assets/male_hero-jump.png").await.unwrap();
    let texture_fall: Texture2D = load_texture("assets/male_hero-fall.png").await.unwrap();

    let frame_width = texture_walk.width();

    // 2. calcular datos básicos
    let sprite_size = 128.0; // todos miden 128x128, falta revisar
    let total_frames = (frame_width / sprite_size) as usize; // 1280 / 128 = 10

    let mut current_frame = 0;
    let mut timer = 0.0;
    let mut position = vec2(300.0, 250.0);
    let mut facing_right = true; // si el jugador mira a la derecha o izda

    // guardamos el estado anterior para saber
    // si se ha movido
    let mut was_moving = false;

    // física y manejo de gravedad
    let mut vel_y = 0.0;
    let mut is_grounded = true;

    loop {
        clear_background(WHITE);
        let dt = get_frame_time();

        // 3. Manejo de Input
        let mut speed = 0.0;

        if is_key_down(KeyCode::Right) {
            speed = 60.0;
            facing_right = true;
        }

        if is_key_down(KeyCode::Left) {
            speed = -60.0;
            facing_right = false;
        }

        //TODO Detectar el salto
        if is_key_pressed(KeyCode::Space) && is_grounded {
            vel_y = JUMP_SPEED;
            is_grounded = false;
            current_frame = 0; // Reiniciamos la animación de salto para que empiece por el principio
            timer = 0.0;
        }

        let is_moving = is_key_down(KeyCode::Right) || is_key_down(KeyCode::Left);

        // Si el estado cambió reiniciamos el frame a 0
        if was_moving != is_moving {
            current_frame = 0;
            timer = 0.0;
        }

        // actualizamos el estado para el
        // próximo fotograma
        was_moving = is_moving;

        // Elegir textura y velocidad según el estado.
        // aqui se valida si se presiona la tecla Shift,
        // para que el jugador "corra".
        // 'frame_speed' cambia el frame cada X segundos,
        // segun si camina o corre.
        // Si me muevo, corro ó camino.
        // Si NO me muevo, hago `idle` (resposo/quieto)
        /*
        let (texture, frame_speed, speed) = if is_key_down(KeyCode::Right) || is_key_down(KeyCode::Left) {
            // Si estamos moviendo el jugador, decidimos entre correr ó caminar
            if is_key_down(KeyCode::LeftShift) {
                let n_speed = if facing_right { speed + 160.0 } else { speed - 160.0 };
                // "corre"
                (&texture_run, 0.08, n_speed)
            } else {
                // "camina"
                (&texture_walk, 0.10, speed)
            }
        } else {
            // estamos en resposo
            // se define una velocidad tranquila para
            // que el jugador "respire"
            (&texture_idle, 0.20, speed)
        };
        */
        // TODO start
        // Elegir textura, velocidad de animación y velocidad de movimiento
        let (texture, frame_speed, speed) = if !is_grounded {
            // ESTAMOS EN EL AIRE
            if vel_y < 0.0 {
                // Subiendo (Salto)
                // Usamos la animación de salto. La ponemos rápida (0.06) y SIN bucle (se congelará al final)
                (&texture_jump, 0.06, speed)
            } else {
                // Bajando (Caída)
                // Usamos la animación de caída. (0.10 para que se vea natural)
                (&texture_fall, 0.10, speed)
            }
        } else {
            // ESTAMOS EN EL SUELO
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::Left) {
                // Movimiento horizontal
                if is_key_down(KeyCode::LeftShift) {
                    let n_speed = if facing_right { speed + 160.0 } else { speed - 160.0 };
                    (&texture_run, 0.08, n_speed)
                } else {
                    (&texture_walk, 0.10, speed)
                }
            } else {
                // Reposo
                (&texture_idle, 0.20, speed)
            }
        };
        // TODO end

        // TODO
        // s1. Aplicar gravedad al eje Y
        vel_y += GRAVITY * dt;

        // s2. Mover en Y
        position.y += vel_y * dt;
        //
        position.x += speed * dt;

        // TODO
        // s3. Detectar el suelo (parte inferior de la ventana)
        if position.y + sprite_size >= screen_height() {
            position.y = screen_height() - sprite_size; // 128 // Ajustamos al borde exacto
            //position.y = screen_height() - 100.0; // Ajustamos al borde exacto
            vel_y = 0.0;
            is_grounded = true;
        }

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
            texture,
            position.x,
            position.y,
            WHITE,
            DrawTextureParams {
                source: Some(source_rect),
                flip_x: !facing_right, // gira el frame si va a la izquierda
                ..Default::default()
            },
        );

        draw_text("Usa FLECHA IZQUIERDA y DERECHA para moverte", 0.0, 20.0, 25.0, BLACK);
        draw_text(&format!("Frame actual: {}", current_frame), 0.0, 35.0, 25.0, BLACK);
        draw_text(format!("toca el suelo: {}", is_grounded).as_str(), 0., 50., 24., BLACK);
        draw_text(
            format!("Player X: {:.1}, y: {:.1}", position.x, position.y).as_str(),
            0.,
            65.,
            24.,
            BLACK,
        );

        next_frame().await;
    }
}

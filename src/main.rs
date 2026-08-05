use macroquad::color::{BLACK, WHITE};
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::math::{Rect, vec2};
use macroquad::text::draw_text;
use macroquad::texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture};
use macroquad::time::get_fps;
use macroquad::time::get_frame_time;
use macroquad::window::{self, clear_background, next_frame, screen_height};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
// La fuerza de gravedad
const GRAVITY: f32 = 600.0;
// velocidad inicial del salto(Empuje inicial hacia arriba)
const JUMP_SPEED: f32 = -350.0;

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
    // cargar la textura(el Sprite Sheet)
    let texture_walk: Texture2D = load_texture("assets/male_hero-walk.png").await.unwrap();
    let texture_run: Texture2D = load_texture("assets/male_hero-run.png").await.unwrap();
    let texture_idle: Texture2D = load_texture("assets/male_hero-idle.png").await.unwrap();
    let texture_jump: Texture2D = load_texture("assets/male_hero-jump.png").await.unwrap();
    let texture_fall: Texture2D = load_texture("assets/male_hero-fall.png").await.unwrap();

    // TO FIX todos miden 128x128 ?
    let sprite_size = 128.0;

    let mut current_frame = 0;
    let mut timer = 0.0;
    let mut position = vec2(300.0, 250.0);
    // si el jugador mira a la derecha o izda
    let mut facing_right = true;

    // guardamos el estado anterior para saber
    // si se ha movido
    let mut was_moving = false;

    // física y manejo de gravedad
    let mut vel_y = 0.0;

    // dependerá de si el jugador empieza en
    // el piso ó en el "aire"
    let mut is_grounded = false;

    loop {
        clear_background(WHITE);
        let dt = get_frame_time();

        // Manejo de Input
        let mut speed = 0.0;

        if is_key_down(KeyCode::Right) {
            speed = 60.0;
            facing_right = true;
        }

        if is_key_down(KeyCode::Left) {
            speed = -60.0;
            facing_right = false;
        }

        if is_key_pressed(KeyCode::Space) && is_grounded {
            vel_y = JUMP_SPEED;
            is_grounded = false;
            // reiniciamos la animación de salto
            // para que empiece por el principio
            // si estaba en el suelo
            if is_grounded {
                current_frame = 0;
                timer = 0.0;
            }
        }

        let is_moving = is_key_down(KeyCode::Right) || is_key_down(KeyCode::Left);

        // si el estado cambió reiniciamos el frame a 0
        if was_moving != is_moving {
            current_frame = 0;
            timer = 0.0;
        }

        // actualizamos el estado para el
        // próximo fotograma
        was_moving = is_moving;

        // elegir textura, velocidad de animación y de
        // movimiento según el estado.
        // aquí se valida si se presiona la tecla Shift,
        // para que el jugador "corra".
        // 'frame_speed' cambia el frame cada X segundos,
        // según si camina o corre.
        // Si me muevo, corro ó camino.
        // Si NO me muevo, hago `idle` (resposo/quieto)
        //
        // estamos en el aire (cayendo?...)
        let (texture, frame_speed, speed) = if !is_grounded {
            // Si estoy en el aire y mi velocidad es 0 (el ápice)
            if vel_y <= 0.0 {
                // o negativa (subiendo?), usamos la animación de salto.
                // La ponemos rápida (frame_speed alto)
                (&texture_jump, 0.07, speed)
            } else {
                // bajando (caída?)
                // Usamos la animación de caída.
                // La ponemos un poco más lenta (frame_speed bajo)
                (&texture_fall, 0.12, speed)
            }
        } else {
            // estamos en el suelo
            // Si estamos moviendo el jugador, decidimos entre correr ó caminar
            if is_key_down(KeyCode::Right) || is_key_down(KeyCode::Left) {
                if is_key_down(KeyCode::LeftShift) {
                    let n_speed = if facing_right { speed + 160.0 } else { speed - 160.0 };
                    // "corre"
                    (&texture_run, 0.04, n_speed)
                } else {
                    // "camina"
                    (&texture_walk, 0.09, speed)
                }
            } else {
                // estamos en resposo
                // se define una velocidad tranquila(más alta)
                // para que el jugador "respire"
                (&texture_idle, 0.20, speed)
            }
        };

        // aplicar gravedad al eje Y
        vel_y += GRAVITY * dt;

        // mover en Y
        position.y += vel_y * dt;
        //
        position.x += speed * dt;

        // detectar el suelo (parte inferior de la ventana)
        // BUG aquí aún no es, el borde inferior de la ventana
        if position.y + sprite_size >= screen_height() {
            position.y = screen_height() - sprite_size; //ajustamos al borde (exacto?)
            vel_y = 0.0;
            is_grounded = true;
            // BUG ó feature?
            // produce un efecto de deslizamiento
            // al moverse.
            //
            // reinicia el frame al tocar suelo
            // current_frame = 0;
            // timer = 0.0;
        }

        // lógica de la animación durante el salto (cambiar el frame)
        timer += dt;

        if timer >= frame_speed {
            timer = 0.0;
            current_frame += 1;

            // calcula el total de frames de la textura
            // que se está dibujando
            let max_frames = (texture.width() / sprite_size) as usize;

            if current_frame >= max_frames {
                if !is_grounded {
                    // si estamos en el aire (jump ó fall)
                    // nos congelamos en el último frame
                    current_frame = max_frames - 1;
                } else {
                    // si estamos en el suelo (walk, run, idle)
                    // hacemos el bucle normal a 0
                    current_frame = 0;
                }
            }
        }

        // cálculo del área de recorte de la textura
        // aqui se obtiene el frame del Sprite Sheet
        let source_x = current_frame as f32 * sprite_size;
        let source_rect = Rect::new(source_x, 0.0, sprite_size, texture.height());

        // dibujar la textura recortada (el frame, usando DrawTextureParams)
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

        // debug info
        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 24., BLACK);
        draw_text("Usa FLECHA IZQUIERDA y DERECHA para moverte", 0.0, 36.0, 25.0, BLACK);
        draw_text(&format!("Frame actual: {}", current_frame), 0.0, 52.0, 25.0, BLACK);
        draw_text(format!("toca el suelo: {}", is_grounded).as_str(), 0., 65., 24., BLACK);
        draw_text(
            format!("Player X: {:.1}, y: {:.1}", position.x, position.y).as_str(),
            0.,
            80.,
            24.,
            BLACK,
        );

        next_frame().await;
    }
}

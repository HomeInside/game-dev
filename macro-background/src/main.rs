#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macro::background".to_owned(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

// crea un fondo a partir de un patrón, generando un mosaico
// de una textura en un RenderTarget una sola vez. Luego,
// en el bucle principal, se usa esa textura pre-renderizada
// como un fondo estático, ahorrando cientos de llamadas de
// dibujo por frame.
// - https://docs.rs/macroquad/latest/macroquad/texture/fn.render_target.html
//
async fn gen_tiled_background(texture: &Texture2D) -> RenderTarget {
    // el tamaño actual de la ventana
    let window_width = screen_width();
    let window_height = screen_height();

    // el ancho de la textura original
    // aqui se asume que el patrón de la textura
    // tiene el mismo ancho y alto
    let tile_size = texture.width();

    // crea un buffer de renderizado en memoria (no visible en pantalla)
    // target es una superficie de dibujo que vive en GPU
    let target = render_target(window_width as u32, window_height as u32);
    target.texture.set_filter(FilterMode::Nearest);

    // hará que el espacio de la cámara sea igual al rectángulo indicado,
    // es decir, el tamaño de la pantalla
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, window_width as f32, window_height as f32));

    // le decimos a Macroquad que todos los dibujos que se hagan mientras
    // esta cámara esté activa se rendericen en el RenderTarget
    // en lugar de en la pantalla principal
    camera.render_target = Some(target.clone());

    // activa la cámaram a partir de aquí, todos los comandos
    // de dibujo van al render target
    set_camera(&camera);

    // limpia el RenderTarget, no la ventana
    clear_background(BLACK);

    // dibujamos el mosaico, se calcula cuántas columnas y filas de
    // tiles se necesitan para cubrir toda la ventana
    // (más uno para asegurar que no haya bordes vacíos)
    let columns = (window_width as f32 / tile_size).ceil() as i32 + 1;
    let rows = (window_height as f32 / tile_size).ceil() as i32 + 1;

    // se dibuja la textura base en cada posición, escalándola al tamaño
    // de `tile_size` (podemos forzar usando `dest_size`)
    // con `WHITE` aseguramos que se dibuje sin modificar el color
    for y in 0..rows {
        for x in 0..columns {
            draw_texture_ex(
                texture,
                x as f32 * tile_size,
                y as f32 * tile_size,
                WHITE,
                DrawTextureParams {
                    //dest_size: Some(vec2(tile_size, tile_size)),
                    ..Default::default()
                },
            );
            //draw_texture(texture, x as f32 * tile_size, y as f32 * tile_size, WHITE);
        }
    }

    // restablecer el modo predeterminado de la cámara 2D
    // esto es importante para que los que se dibuje después
    // no vaya al render target, que estamos creando
    set_default_camera();

    target
}

#[macroquad::main(window_conf)]
async fn main() -> Result<(), macroquad::Error> {
    let texture_pattern = load_texture("Blue.png").await?;
    let fondo = load_texture("background_00.png").await?;

    let render_background = gen_tiled_background(&texture_pattern).await;
    let mut set_back = 1;

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("click izquierdo");
            set_back = 1;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            println!("click derecho");
            set_back = 2;
        }

        if set_back == 1 {
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
            draw_text("Fondo normal con imagen", 20.0, 30.0, 20.0, BLACK);
        } else {
            draw_texture_ex(
                &render_background.texture,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    //flip_y: true,
                    ..Default::default()
                },
            );
            draw_text("Fondo en mosaico (tiled)", 20.0, 30.0, 20.0, BLACK);
        }

        next_frame().await;
    }
}

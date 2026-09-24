use gilrs::ff::{BaseEffect, BaseEffectType, Effect, EffectBuilder, Replay, Ticks};
use gilrs::{Axis, Button, EventType, GamepadId, Gilrs};

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;
const CIRCLE_RADIUS: f32 = 30.0;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macroquad :: gamepad".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        high_dpi: true,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gilrs = Gilrs::new().expect("No se pudo inicializar GilRs");
    // construir el efecto de rumble(ff)
    let mut collision_effect = build_collision_effect(&mut gilrs);

    // tiempo restante antes de poder vibrar de nuevo
    let mut cooldown: f32 = 0.0;

    loop {
        while let Some(event) = gilrs.next_event() {
            // manejar los eventos del gamepad
            match event.event {
                EventType::Connected => {
                    println!("Gamepad conectado: {:?}", event.id);
                    // construir el efecto si aún no lo tenemos
                    if collision_effect.is_none() {
                        collision_effect = build_collision_effect(&mut gilrs);
                    }
                }

                EventType::Disconnected => {
                    println!("Gamepad desconectado: {:?}", event.id);
                    collision_effect = None;
                }

                EventType::ButtonPressed(button, _) => {
                    println!("Botón presionado: {:?}", button);
                    if button == Button::North {
                        println!("Y -> ^");
                    }
                    if button == Button::South {
                        println!("A -> x");
                    }
                    if button == Button::East {
                        println!("B -> o");
                    }
                    if button == Button::West {
                        println!("X -> ▀");
                    }
                }

                EventType::ButtonReleased(button, _) => {
                    println!("Botón soltado: {:?}", button);
                }

                EventType::AxisChanged(axis, value, _) => {
                    println!("EJE: {:?} = {}", axis, value);
                }

                _ => {}
            }
        }

        clear_background(BLACK);
        if let Some((id, gamepad)) = gilrs.gamepads().next() {
            //draw_hud_gamepad(&gamepad, id);

            // START
            // con deadzone
            let lx = deadzone(gamepad.value(Axis::LeftStickX), 0.15);
            let ly = deadzone(gamepad.value(Axis::LeftStickY), 0.15);
            let rx = deadzone(gamepad.value(Axis::RightStickX), 0.15);
            let ry = deadzone(gamepad.value(Axis::RightStickY), 0.15);

            /*
            // sin deadzone
            let lx = gamepad.value(Axis::LeftStickX);
            let ly = gamepad.value(Axis::LeftStickY);
            let rx = gamepad.value(Axis::RightStickX);
            let ry = gamepad.value(Axis::RightStickY);
            */

            // mostramos la info
            draw_text(&format!("Gamepad name: {}", gamepad.name()), 20.0, 30.0, 30.0, WHITE);
            draw_text(&format!("ID: {:?}", id), 20.0, 50.0, 30.0, WHITE);

            draw_text(&format!("Left stick: {:.2}, {:.2}", lx, ly), 20.0, 90.0, 30.0, WHITE);
            draw_text(&format!("Right Stick: {:.2}, {:.2}", rx, ry), 20.0, 110.0, 30.0, WHITE);

            let power = gamepad.power_info();
            draw_text(&format!("power_info: {:?}", power), 20.0, 140.0, 30.0, WHITE);

            let ff_status = if collision_effect.is_some() {
                "FF(vibración): Activo"
            } else {
                "FF(vibración): no disponible"
            };

            draw_text(ff_status, 20.0, 170.0, 30.0, WHITE);

            //
            // obtenemos las coordenadas para mover los circulos
            let player_lx = 200.0 + lx * 200.0;
            let player_ly = screen_height() / 2.0 + ly * 200.0;

            let player_rx = 600.0 + rx * 200.0;
            let player_ry = screen_height() / 2.0 + ry * 200.0;

            draw_circle(player_lx, player_ly, CIRCLE_RADIUS, RED);
            draw_circle(player_rx, player_ry, CIRCLE_RADIUS, BLUE);

            // detectar la colisión
            let dx = player_lx - player_rx;
            let dy = player_ly - player_ry;
            let distance = (dx * dx + dy * dy).sqrt();

            // para que colisione, la distancia entre centros
            // debe ser menor a la suma de radios
            let colliding = distance < CIRCLE_RADIUS * 2.0;

            if colliding && cooldown <= 0.0 {
                // activar el rumble
                if let Some(effect) = &collision_effect {
                    if let Err(e) = effect.play() {
                        println!("Error al reproducir rumble: {}", e);
                    }
                }

                // reiniciamos el cooldown (0.5s) para no
                // vibrar en cada frame
                cooldown = 0.5;
            }

            // si colisiona
            if colliding {
                draw_circle_lines(player_lx, player_ly, CIRCLE_RADIUS + 3.0, 2.0, GREEN);
                draw_circle_lines(player_rx, player_ry, CIRCLE_RADIUS + 3.0, 2.0, GREEN);
            }
        } else {
            draw_text("Conecta un gamepad...", 20.0, 40.0, 30.0, WHITE);
        }
        next_frame().await;
    }
}

/// evitar el drift (deriva), es decir el personaje
// ó el cursor se mueve solo aunque no toques el control, para esto
// se implementa la `deadzone`, un umbral mínimo para definir si el
// valor del stick está por debajo y devolver cero(0), solo se
// moverá cuando se mueva el stick de verdad.
fn deadzone(value: f32, threshold: f32) -> f32 {
    if value.abs() < threshold { 0.0 } else { value }
}

/// construye el efecto de rumble asociado a todos los
// gamepads que lo soporten., puede retornar `None` si
// ningún gamepad lo soporta.
fn build_collision_effect(gilrs: &mut Gilrs) -> Option<Effect> {
    // 1. Encontrar los Gamepad's con soporte de force feedback
    let ff_ids: Vec<GamepadId> = gilrs
        .gamepads()
        .filter_map(|(id, gp)| if gp.is_ff_supported() { Some(id) } else { None })
        .collect();

    if ff_ids.is_empty() {
        // Force feedback no soportado en este
        // gamepad (o no hay gamepad conectado)
        return None;
    }

    println!("Force feedback disponible en: {:?}", ff_ids);

    // efecto de vibración corto y fuerte
    let duration = Ticks::from_ms(5);

    let effect = EffectBuilder::new()
        .add_effect(BaseEffect {
            kind: BaseEffectType::Strong { magnitude: 5_000 },
            scheduling: Replay {
                play_for: duration,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_effect(BaseEffect {
            kind: BaseEffectType::Weak { magnitude: 1_000 },
            scheduling: Replay {
                play_for: duration,
                ..Default::default()
            },
            ..Default::default()
        })
        .gamepads(&ff_ids)
        .finish(gilrs)
        .expect("Error al construir el efecto de force feedback");

    Some(effect)
}

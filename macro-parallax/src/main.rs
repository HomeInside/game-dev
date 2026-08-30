// hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use macroquad::window::{self, next_frame};

pub struct VerticalParallax {
    background: Texture2D,
    height: f32,
    speed: f32,
    offset: f32,
    scale_x: f32,
}

impl VerticalParallax {
    pub fn new(background: Texture2D, speed: f32, scale_x: f32) -> Self {
        let height = background.height() as f32;

        Self {
            background,
            height,
            speed,
            offset: 0.0,
            scale_x,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // mueve hacia abajo
        self.offset += self.speed * dt;

        // cuando la imagen se desplaza hacia
        // abajo y sale por la parte inferior,
        // la "saltamos" hacia arriba
        // (restando el alto), así se mantiene en el rango
        if self.offset >= self.height {
            self.offset -= self.height;
        }
    }

    pub fn draw(&self) {
        // dibuja imagen 1
        draw_texture_ex(
            &self.background,
            0.0,
            self.offset,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.scale_x, self.height)),
                ..Default::default()
            },
        );

        // dibuja imagen 2 (copia debajo)
        draw_texture_ex(
            &self.background,
            0.0,
            self.offset - self.height,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.scale_x, self.height)),
                ..Default::default()
            },
        );
    }
}

pub struct HorizontalParallax {
    background: Texture2D,
    width: f32,
    speed: f32,
    offset: f32,
    #[allow(dead_code)]
    overlap: f32,
    scale_y: f32,
}

impl HorizontalParallax {
    pub fn new(background: Texture2D, speed: f32, scale_y: f32) -> Self {
        let width = background.width() as f32;

        Self {
            background,
            width,
            speed,
            offset: 0.0,
            overlap: 1.0,
            scale_y,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // mueve de izquierda a derecha
        self.offset -= self.speed * dt;

        // cuando la imagen se desplaza tanto
        // que sale completamente por la izquierda,
        // la "saltamos" de vuelta a la derecha
        // (sumando el ancho), así se mantiene en el rango
        if self.offset <= -self.width {
            self.offset += self.width;
        }
    }

    pub fn draw(&self) {
        // dibuja imagen 1
        draw_texture_ex(
            &self.background,
            self.offset,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.scale_y)),
                ..Default::default()
            },
        );

        // dibuja imagen 2 (copia a la derecha)
        draw_texture_ex(
            &self.background,
            self.offset + self.width,
            //self.offset + self.width - self.overlap,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.scale_y)),
                ..Default::default()
            },
        );
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macro::parallax".to_owned(),
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
    let scale_x = screen_width();
    let scale_y = screen_height();

    let h_backg = load_texture("horizontal_bg.png").await?;
    let v_backg = load_texture("vertical_bg.png").await?;

    let mut vertical_parallax = VerticalParallax::new(v_backg, 100.0, scale_x);

    let mut horizontal_parallax = HorizontalParallax::new(h_backg, 100.0, scale_y);

    let mut set_back = 1;

    loop {
        clear_background(WHITE);
        let dt = get_frame_time();

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("click izquierdo");
            set_back = 1;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            println!("click derecho");
            set_back = 2;
        }

        if set_back == 1 {
            horizontal_parallax.update(dt);
            horizontal_parallax.draw();
        } else {
            vertical_parallax.update(dt);
            vertical_parallax.draw();
        }

        next_frame().await;
    }
}

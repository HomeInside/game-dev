use macroquad::color::{BLACK, WHITE};
use macroquad::file::load_string;
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::math::{Rect, vec2};
use macroquad::text::draw_text;
use macroquad::texture::{DrawTextureParams, FilterMode, Texture2D, draw_texture_ex, load_texture};
//use macroquad::tiled::load_map;
use macroquad::time::get_fps;
use macroquad::time::get_frame_time;
use macroquad::window::{self, clear_background, next_frame, screen_height, screen_width};
use macroquad_tiled as tiled;

const WIDTH: i32 = 1024; //800;
const HEIGHT: i32 = 768; //600;
// La fuerza de gravedad
const GRAVITY: f32 = 600.0;
// velocidad inicial del salto(Empuje inicial hacia arriba)
const JUMP_SPEED: f32 = -350.0;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macroquad :: tilesed".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let tileset = load_texture("assets/terrain.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/mapa_export.json").await.unwrap();
    println!("{}", &tiled_map_json[0..300]);
    let tiled_map =
        tiled::load_map(&tiled_map_json, &[("terrain.png", tileset.clone())], &[]).expect("error cargando mapa");

    let mut current_frame = 0;

    // dependerá de si el jugador empieza en
    // el piso ó en el "aire"
    let mut is_grounded = false;
    let mut position = vec2(300.0, 250.0);
    let map_width = tiled_map.raw_tiled_map.width * tiled_map.raw_tiled_map.tilewidth;

    let map_height = tiled_map.raw_tiled_map.height * tiled_map.raw_tiled_map.tileheight;
    println!("tileset tamaño: {}x{}", &tileset.width(), &tileset.height());

    println!("layers: {:?}", tiled_map.layers.keys());
    println!("tilesets: {:?}", tiled_map.tilesets.keys());

    for (name, layer) in &tiled_map.layers {
        println!(
            "layer={} width={} height={} tiles={}",
            name,
            layer.width,
            layer.height,
            layer.data.len()
        );
    }
    //
    for (name, layer) in &tiled_map.layers {
        let valid_tiles = layer.data.iter().filter(|t| t.is_some()).count();

        println!("{}: {}/{} tiles válidos", name, valid_tiles, layer.data.len());
    }
    //
    let layer = &tiled_map.layers["terrain"];

    let mut count = 0;

    for tile in &layer.data {
        if tile.is_some() {
            count += 1;
        }
    }

    println!("tiles reales cargados: {}", count);

    loop {
        clear_background(WHITE);
        let dt = get_frame_time();
        //draw_texture_ex(&tileset, 0.0, 0.0, WHITE, DrawTextureParams { ..Default::default() });

        tiled_map.draw_tiles(
            "baselayer1",
            //Rect::new(0.0, 0.0, map_width as f32, map_height as f32),
            Rect::new(0.0, 0.0, screen_width(), screen_height()),
            None,
        );
        tiled_map.draw_tiles(
            "terrain",
            //Rect::new(0.0, 0.0, map_width as f32, map_height as f32),
            Rect::new(0.0, 0.0, screen_width(), screen_height()),
            None,
        );

        next_frame().await;
    }
}

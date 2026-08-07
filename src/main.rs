use macroquad::color::{BLACK, RED, WHITE};
use macroquad::file::load_string;
use macroquad::math::{Rect /*vec2*/};
use macroquad::texture::{FilterMode, load_texture};
//use macroquad::time::get_frame_time;
use macroquad::prelude::*;
use macroquad::shapes::draw_rectangle_lines;
use macroquad::window::{self, clear_background, next_frame, screen_height, screen_width};
mod macroquad_tiled;
use macroquad_tiled as tiled;

const WIDTH: i32 = 1024; //800;
const HEIGHT: i32 = 768; //600;
// La fuerza de gravedad
//const GRAVITY: f32 = 600.0;
// velocidad inicial del salto(Empuje inicial hacia arriba)
//const JUMP_SPEED: f32 = -350.0;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "macroquad :: tilesed".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
}
/*
fn draw_polygon(obj: &macroquad_tiled::tiled::layer::Object) {
    for obj in &raw_layer.objects {
        if let Some(points) = &obj.polygon {
            for i in 0..points.len() {
                let a = &points[i];
                let b = &points[(i + 1) % points.len()];

                draw_line(obj.x + a.x, obj.y + a.y, obj.x + b.x, obj.y + b.y, 2.0, RED);
            }
        }
    }
}*/

#[macroquad::main(window_conf)]
async fn main() {
    let tileset = load_texture("assets/terrain.png").await.unwrap();
    tileset.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/mapa_export.json").await.unwrap();
    println!("{}", &tiled_map_json[0..300]);

    // para los los objetos cambia de 'objectgroup' a 'objectlayer' en el json

    let tiled_map =
        tiled::load_map(&tiled_map_json, &[("terrain.png", tileset.clone())], &[]).expect("error cargando mapa");

    //let mut current_frame = 0;

    // dependerá de si el jugador empieza en
    // el piso ó en el "aire"
    //let mut is_grounded = false;
    //let mut position = vec2(300.0, 250.0);
    //let map_width = tiled_map.raw_tiled_map.width * tiled_map.raw_tiled_map.tilewidth;
    //let map_height = tiled_map.raw_tiled_map.height * tiled_map.raw_tiled_map.tileheight;
    println!("tileset tamaño: {}x{}", &tileset.width(), &tileset.height());

    println!("layers: {:?}", tiled_map.layers.keys());
    println!("tilesets: {:?}", tiled_map.tilesets.keys());

    // info del mapa
    for (name, layer) in &tiled_map.layers {
        println!(
            "layer={} width={} height={} tiles={}",
            name,
            layer.width,
            layer.height,
            layer.data.len()
        );
    }

    // info de los tiles
    for (name, layer) in &tiled_map.layers {
        let valid_tiles = layer.data.iter().filter(|t| t.is_some()).count();

        println!("{}: {}/{} tiles válidos", name, valid_tiles, layer.data.len());
    }

    let layer = &tiled_map.layers["objects1"];
    // objetos del mapa
    println!("Objetos: {}", layer.objects.len());
    println!();

    for obj in &layer.objects {
        //println!("objeto id={}", obj.id);
        if let Some(gid) = obj.gid {
            println!("objeto tile gid={} x={} y={}", gid, obj.world_x, obj.world_y);
        }
    }

    println!();
    println!("raw_tiled_map");
    // raw_tiled_map
    /*
    for layer in &tiled_map.raw_tiled_map.layers {
        if layer.name == "objects1" {
            for object in &layer.objects {
                println!("{:#?}", object);
            }
        }
    }
    */
    /*for layer in &tiled_map.raw_tiled_map.layers {
        println!("RAW layer={}", layer.name);

        println!("{:#?}", layer.objects);
        /*for obj in &layer.objects {
            //println!("{:#?}", obj);
            println!("{:#?}", obj.polygon);
        }*/
    }*/
    for layer in &tiled_map.raw_tiled_map.layers {
        if layer.name == "objects1" {
            println!("{}", std::any::type_name_of_val(&layer.objects[0]));
        }
        for obj in &layer.objects {
            println!("{:?}", obj);
            //println!("{} {}", obj.x, obj.y);

            if let Some(points) = &obj.polygon {
                println!("puntos: {}", points.len());
            }
        }
    }
    println!();
    let raw_layer = tiled_map
        .raw_tiled_map
        .layers
        .iter()
        .find(|l| l.name == "objects1")
        .unwrap();

    loop {
        clear_background(BLACK);

        // cargar el layer base
        tiled_map.draw_tiles("baselayer1", Rect::new(0.0, 0.0, screen_width(), screen_height()), None);

        // cargar el layer del mapa ppal
        tiled_map.draw_tiles("terrain", Rect::new(0.0, 0.0, screen_width(), screen_height()), None);

        // gid objects tiles
        for obj in &layer.objects {
            if let Some(gid) = obj.gid {
                let tile_id = gid - 1;

                tiled_map.spr(
                    "terrain",
                    tile_id,
                    Rect::new(obj.world_x, obj.world_y - obj.world_h, obj.world_w, obj.world_h),
                );
            }
        }

        for obj in &raw_layer.objects {
            //poligonos
            if let Some(points) = &obj.polygon {
                //cerrados
                for i in 0..points.len() {
                    let a = &points[i];
                    let b = &points[(i + 1) % points.len()];

                    draw_line(obj.x + a.x, obj.y + a.y, obj.x + b.x, obj.y + b.y, 2.0, RED);
                }
                // abiertos
                /*
                for i in 0..points.len() - 1 {
                    let a = &points[i];
                    let b = &points[i + 1];

                    draw_line(obj.x + a.x, obj.y + a.y, obj.x + b.x, obj.y + b.y, 2.0, RED);
                }
                */
            }
            //capsule
            if obj.ty == "Capsule" {
                let r = obj.height / 2.0;

                draw_line(obj.x + r, obj.y, obj.x + obj.width - r, obj.y, 2.0, GREEN);

                draw_line(
                    obj.x + r,
                    obj.y + obj.height,
                    obj.x + obj.width - r,
                    obj.y + obj.height,
                    2.0,
                    GREEN,
                );

                draw_circle_lines(obj.x + r, obj.y + r, r, 2.0, GREEN);
                draw_circle_lines(obj.x + obj.width - r, obj.y + r, r, 2.0, GREEN);
            }

            //cirulos
            /*if obj.ellipse.is_some() {
                draw_circle_lines(
                    obj.x + obj.width / 2.0,
                    obj.y + obj.height / 2.0,
                    obj.width.min(obj.height) / 2.0,
                    2.0,
                    RED,
                );
            }*/
            // objeto ellipse nativo de Tiled
            if obj.ellipse == Some(true) {
                draw_ellipse(
                    obj.x + obj.width / 2.0,
                    obj.y + obj.height / 2.0,
                    obj.width / 2.0,
                    obj.height / 2.0,
                    0.0,
                    RED,
                );
            }
            // tus tipos personalizados
            else if obj.ty == "Elipse" {
                draw_ellipse(
                    obj.x + obj.width / 2.0,
                    obj.y + obj.height / 2.0,
                    obj.width / 2.0,
                    obj.height / 2.0,
                    0.0,
                    RED,
                );
            }

            //elipse
            /*if obj.ty == "elipse" {
                draw_ellipse(
                    obj.x + obj.width / 2.0,
                    obj.y + obj.height / 2.0,
                    obj.width / 2.0,
                    obj.height / 2.0,
                    0.0,
                    RED,
                );
            }*/
            if obj.ty == "Elipse" {
                draw_ellipse(
                    obj.x + obj.width / 2.0,
                    obj.y + obj.height / 2.0,
                    obj.width / 2.0,
                    obj.height / 2.0,
                    0.0,
                    RED,
                );
            }

            //puntos
            /*
            if obj.width == 0.0 && obj.height == 0.0 {
                draw_circle(obj.x, obj.y, 4.0, YELLOW);
            }
            */
            // puntos
            /*if obj.width == 0.0
                && obj.height == 0.0
                && obj.polygon.is_none()
                && obj.ellipse.is_none()
            {
                draw_circle(obj.x, obj.y, 4.0, YELLOW);
            }*/
            if obj.ty == "Point" {
                draw_circle(obj.x, obj.y, 4.0, YELLOW);
            }

            //rectangulos
            if obj.gid.is_none()
                && obj.ellipse.is_none()
                && obj.polygon.is_none()
                && obj.ty != "Capsule"
                && obj.name != "Elipse"
                && obj.width > 0.0
                && obj.height > 0.0
            {
                draw_rectangle_lines(obj.x, obj.y, obj.width, obj.height, 2.0, RED);
            }

            // aun no se conserva el campo text del objeto Tiled.
            if obj.ty == "Label" {
                draw_text("Hola Mundo", obj.x, obj.y + obj.height, 24.0, WHITE);
            }
            /*if obj.ty == "label" {
                for prop in &obj.properties {
                    if prop.name == "text" {
                        if let tiled::PropertyValue::String(value) = &prop.value {
                            draw_text(value, obj.x, obj.y + obj.height, 24.0, WHITE);
                        }
                    }
                }
            }
            */
        }

        next_frame().await;
    }
}

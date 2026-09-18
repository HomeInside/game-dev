#include "raylib.h"
#include <array>
#include <format>
#include <iostream>
#include <string>

constexpr int SCREEN_WIDTH = 920;
constexpr int SCREEN_HEIGHT = 600;
constexpr std::size_t MAX_ITEMS = 3;

int main() {
  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "raylib :: mouse");

  //  cargar las cat_images
  std::array<Texture2D, MAX_ITEMS> cat_images{
      LoadTexture("ray_mouse/assets/c1_head.png"),
      LoadTexture("ray_mouse/assets/c9_head.png"),
      LoadTexture("ray_mouse/assets/c11_head.png"),
  };

  // crear los rectangulos de origen
  std::array<Rectangle, MAX_ITEMS> origin_box{Rectangle{40, 40, 89, 84},
                                              Rectangle{40, 150, 117, 79},
                                              Rectangle{40, 250, 89, 84}};

  // crear los rectangulos de destino
  std::array<Rectangle, MAX_ITEMS> destiny_box{Rectangle{200, 50, 117, 84},
                                               Rectangle{200, 200, 117, 84},
                                               Rectangle{200, 320, 117, 84}};

  // este array nos permite definir que cajas
  // de destino contienen items, usamos el
  // valor de `-1` para indicar que el destino
  // está vacío
  std::array<int, MAX_ITEMS> contenidoDestino{-1, -1, -1};

  // usamos el valor de `-1` para indicar
  // que que no hay ningun item seleccionado
  int item_selected = -1;

  // manejo del ratón
  bool has_click = false;
  bool l_click = false;
  bool r_click = false;
  Vector2 prev_mouse_pos;

  // tiempo para manejar los mensajes
  float time = 0.0f;

  while (!WindowShouldClose()) {
    Vector2 mouse = GetMousePosition();
    float dt = GetFrameTime();
    time += dt;

    //
    // manejar el click izquierdo
    if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
      bool click_in_item = false;
      // guardar la posicion del mouse
      prev_mouse_pos = mouse;
      // fijar a true, para activar el mensaje
      has_click = true;
      l_click = true;
      // resetear el tiempo
      time = 0.0;

      // recorremos las `origin_box` para validar a quien
      // se le hizo click
      for (std::size_t i = 0; i < std::size(origin_box); ++i) {
        if (CheckCollisionPointRec(mouse, origin_box[i])) {
          item_selected = i;
          click_in_item = true;

          // si encontramos, salimos del for
          break;
        }
      }

      // recorremos las `destiny_box` para validar a quien
      // se le hizo click
      if (!click_in_item) {
        for (std::size_t i = 0; i < std::size(destiny_box); ++i) {
          if (CheckCollisionPointRec(mouse, destiny_box[i])) {
            click_in_item = true;

            // si tenemos una imagen seleccionada,
            // la colocamos en el destino.
            if (item_selected != -1) {
              contenidoDestino[i] = item_selected;

              // podemos desmarcar el item seleccionado
              // ó comentalo para permitir seguir haciendo click en
              // las cajas de destino
              item_selected = -1;
            }

            break;
          }
        }
      }

      // si se hizo click por fuera de las cajas
      // desmarcamos el item seleccionado
      if (!click_in_item) {
        item_selected = -1;
      }
    }

    // manejar el click derecho
    // se utiliza para limpiar la caja de destino
    if (IsMouseButtonPressed(MOUSE_BUTTON_RIGHT)) {
      // guardar la posicion del mouse
      prev_mouse_pos = mouse;
      // fijar a true, para activar el mensaje
      has_click = true;
      r_click = true;
      // resetear el tiempo
      time = 0.0;

      // recorremos las `destiny_box` para validar a quien
      // se le hizo click
      for (std::size_t i = 0; i < std::size(destiny_box); ++i) {
        if (CheckCollisionPointRec(mouse, destiny_box[i])) {
          // desmarcar cada item seleccionado
          contenidoDestino[i] = -1;

          break;
        }
      }
    }

    BeginDrawing();

    ClearBackground(RAYWHITE);

    // dibujar las `cat_images`
    for (std::size_t i = 0; i < std::size(cat_images); ++i) {
      Rectangle source = {0.0f, 0.0f, static_cast<float>(cat_images[i].width),
                          static_cast<float>(cat_images[i].height)};

      Vector2 origin = {0.0f, 0.0f};

      DrawTexturePro(cat_images[i], source, origin_box[i], origin, 0.0f, WHITE);
    }

    // dibujar las `destiny_box`
    for (std::size_t i = 0; i < std::size(destiny_box); ++i) {
      // dibujar borde del espacio
      DrawRectangleLinesEx(destiny_box[i], 3, DARKGRAY);

      // si hay una imagen seleccionada
      if (contenidoDestino[i] != -1) {
        int idImagen = contenidoDestino[i];
        Rectangle source = {0.0f, 0.0f,
                            static_cast<float>(cat_images[idImagen].width),
                            static_cast<float>(cat_images[idImagen].height)};
        Vector2 origin = {0.0f, 0.0f};

        // dibujarla en el destino
        DrawTexturePro(cat_images[idImagen], source, destiny_box[i], origin,
                       0.0f, WHITE);
      }
    }

    // Resaltar de color verde la `origin_box`
    // con el item seleccionado
    if (item_selected != -1) {
      DrawRectangleLinesEx(origin_box[item_selected], 5, GREEN);
    }

    if (has_click) {
      time += dt;
      // mientras el tiempo sea menor que 2 segundos
      // mostrar el mensaje
      if (time < 2.0) {
        /*
        std::cout << std::format("le dio click X: {:.3f}", prev_mouse_pos.x)
                  << std::endl;
        std::cout << std::format("le dio click Y: {:.3f}", prev_mouse_pos.y)
                  << std::endl;
        */

        if (l_click) {
          const std::string click_text =
              std::format("click izquierdo en: X: {:.2f}, Y: {:.2f}",
                          prev_mouse_pos.x, prev_mouse_pos.y);
          DrawText(click_text.c_str(), 30, 10, 18, RED);
        }
        if (r_click) {
          const std::string click_text =
              std::format("click derecho en: X: {:.2f}, Y: {:.2f}",
                          prev_mouse_pos.x, prev_mouse_pos.y);
          DrawText(click_text.c_str(), 30, 25, 18, BLACK);
        }
      } else {
        // cuando llegue a 2 segundos
        // desactivar el mensaje
        has_click = false;
        l_click = false;
        r_click = false;
      }
    }

    EndDrawing();
  }

  UnloadTexture(cat_images[0]);
  UnloadTexture(cat_images[1]);
  UnloadTexture(cat_images[2]);

  CloseWindow();

  return 0;
}

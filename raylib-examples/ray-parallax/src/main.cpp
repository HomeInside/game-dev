#include "raylib.h"

#include <string>

constexpr int SCREEN_WIDTH = 800;
constexpr int SCREEN_HEIGHT = 600;

struct VerticalParallax {
  Texture2D background{};
  float height{0.0f};
  float speed{100.0f};
  float offset{0.0f};
  float scale_x{0.0f};
  float scale_y{0.0f};

  VerticalParallax(const Image &source, float speed, float scale_x,
                   float scale_y)
      : background(LoadTextureFromImage(source)), height(source.width),
        speed(speed), offset(0.0f), scale_x(scale_x), scale_y(scale_y) {}

  // destructor
  ~VerticalParallax() {
    // Once image has been converted to texture
    // and uploaded to VRAM, it can be unloaded
    // from RAM
    UnloadTexture(background);
  }

  void update(float dt) {
    // mueve hacia abajo
    offset += speed * dt;

    // cuando la imagen se desplaza hacia
    // abajo y sale por la parte inferior,
    // la "saltamos" hacia arriba
    // (restando el alto), así se mantiene
    // en el rango

    // si se utiliza DrawTexture
    if (offset >= height) {
      offset -= height;
    }

    // si se utiliza DrawTexturePro
    /*
    if (offset >= scale_y) {
        offset -= scale_y;
    }*/
  }

  // cuando se necesita dibujar la textura sin
  // cambiar su tamaño
  void draw() const {
    // DrawTexture dibuja la textura en (x, y) usando su tamaño original

    // dibuja imagen 1
    DrawTexture(background, 0, static_cast<int>(offset), WHITE);

    // dibuja imagen 2 (copia debajo)
    float new_pos = offset - height;
    DrawTexture(background, 0, static_cast<int>(new_pos), WHITE);
  }

  // cuando se necesita transformar/escalar la textura al dibujarla.
  void draw_ex() const {
    // un Rectangle con el tamaño de la textura
    Rectangle source = {0.0f, 0.0f, static_cast<float>(background.width),
                        static_cast<float>(background.height)};

    Vector2 origin = {0.0f, 0.0f};

    // tamaño y posición en pantalla (destino)
    Rectangle dest1 = {
        0.0f, offset,
        scale_x, // ancho de la ventana
        scale_y  // alto de la ventana
    };

    Rectangle dest2 = {
        0.0f, offset - scale_y,
        scale_x, // ancho de la ventana
        scale_y  // alto de la ventana
    };

    // DrawTexturePro permite transformar/escalar la textura al dibujarla.

    // Imagen 1
    DrawTexturePro(background, source, dest1, origin, 0.0f, WHITE);

    // dibuja imagen 2 (copia debajo)
    DrawTexturePro(background, source, dest2, origin, 0.0f, WHITE);
  }
};

// se dibujan texto y
// cualquier otra info
void draw_hud() {
  // dibujar los FPS
  // DrawFPS(10, 10);
  const std::string fps_text = "FPS: " + std::to_string(GetFPS());
  DrawText(fps_text.c_str(), 10, 10, 18, RED);
}

//-------------------------------
// raylib 6.0 (Apr 23 2026)
//-------------------------------
int main(void) {
  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "raylib :: parallax");

  // NOTE: Textures MUST be loaded after
  // Window initialization (OpenGL context is required)
  Image image = LoadImage("vertical_bg.png");

  float scale_x = GetScreenWidth();
  float scale_y = GetScreenHeight();

  VerticalParallax v_p = {
      image,
      100.0f,
      scale_x,
      scale_y,
  };

  SetTargetFPS(60);

  // Main game loop
  while (!WindowShouldClose()) {
    float dt = GetFrameTime();

    //
    // input/update
    //

    //
    // Update
    //
    v_p.update(dt);

    //
    // Draw
    //
    BeginDrawing();
    //

    ClearBackground(RAYWHITE);
    v_p.draw();

    draw_hud();

    //
    // End Draw
    //
    EndDrawing();
    //
  }

  // Close window and OpenGL context
  CloseWindow();

  return 0;
}

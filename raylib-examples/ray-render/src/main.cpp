#include "raylib.h"

#include <string>

constexpr int SCREEN_WIDTH = 800;
constexpr int SCREEN_HEIGHT = 600;

// se dibujan texto y
// cualquier otra info
void draw_hud() {
  // dibujar los FPS
  // DrawFPS(10, 10);
  const std::string fps_text = "FPS: " + std::to_string(GetFPS());
  DrawText(fps_text.c_str(), 10, 10, 18, RED);
}

// crea un fondo a partir de un patrón, generando un mosaico
// de una textura en un RenderTarget una sola vez. Luego,
// en el bucle principal, se usa esa textura pre-renderizada
// como un fondo estático, ahorrando cientos de llamadas de
// dibujo por frame.
RenderTexture2D gen_tiled_background(const Image &source) {
  // creamos una textura a partir de la imagen
  Texture2D pattern = LoadTextureFromImage(source);

  // el ancho de la imagen original
  // aqui se asume que el patrón de la textura
  // tiene el mismo ancho y alto
  size_t tile_size = source.width;

  // crea un buffer de renderizado en memoria (no visible en pantalla)
  // target es una superficie de dibujo que vive en GPU
  RenderTexture2D target = LoadRenderTexture(SCREEN_WIDTH, SCREEN_HEIGHT);

  // partir de aquí, todos los comandos
  // de dibujo van al render target
  BeginTextureMode(target);

  // limpia el RenderTarget, no la ventana
  ClearBackground(WHITE);

  // se dibuja la textura base en cada posición, escalándola
  // al tamaño de `tile_size`, con `WHITE` aseguramos que se
  // dibuje sin modificar el color

  for (int y = 0; y < SCREEN_HEIGHT; y += tile_size) {
    for (int x = 0; x < SCREEN_WIDTH; x += tile_size) {
      DrawTexture(pattern, x, y, WHITE);
    }
  }

  // restablecer el modo predeterminado esto es
  // importante para que los que se dibuje después
  // no vaya al render target, que estamos creando
  EndTextureMode();

  return target;
}

//-------------------------------
// raylib 6.0 (Apr 23 2026)
//-------------------------------
int main(void) {
  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "raylib :: render-target");

  // NOTE: Textures MUST be loaded after
  // Window initialization (OpenGL context is required)
  Image image = LoadImage("background_1.png");

  RenderTexture2D render_bg = gen_tiled_background(image);

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

    //
    // Draw
    //
    BeginDrawing();
    //

    ClearBackground(RAYWHITE);

    DrawTexture(render_bg.texture, 0, 0, WHITE);

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

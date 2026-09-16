#include "raylib-tileson.h"
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
  DrawText(fps_text.c_str(), 10, 10, 18, WHITE);
}

int main(int argc, char *argv[]) {
  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "[raylib-tileson] example");
  SetTargetFPS(60);

  // carga el mapa desde Json
  Map map = LoadTiled("ray_tileset/assets/mapa_export.json");

  while (!WindowShouldClose()) {
    BeginDrawing();
    {
      ClearBackground(RAYWHITE);

      // dibuja el mapa
      DrawTiled(map, 0, 0, WHITE);

      // dibuja el HUD
      draw_hud();
    }

    EndDrawing();
  }

  // Descarga los datos del mapa
  UnloadMap(map);

  CloseWindow();

  return 0;
}

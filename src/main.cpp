#include "raylib.h"
// #include <array>

constexpr int SCREEN_WIDTH = 800;
constexpr int SCREEN_HEIGHT = 600;

//-----------------------------
// raylib 6.0 (Apr 23 2026)
//-----------------------------
int main() {
  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "raylib :: sprite");

  while (!WindowShouldClose()) {
    float dt = GetFrameTime();

    BeginDrawing();

    ClearBackground(RAYWHITE);

    EndDrawing();
  }

  CloseWindow();

  return 0;
}

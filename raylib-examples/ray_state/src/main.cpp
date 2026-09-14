#include "config.hpp"
#include "game/game.hpp"
#include "raylib-cpp.hpp"

//-----------------------------
// raylib 6.0 (Apr 23 2026)
// with raylib-cpp 6.0.3 (Aug 2 2026)
//-----------------------------

// Disable the console in Windows releases
// -
// https://keasigmadelta.com/blog/raylib-hide-the-console-window-on-windows-release-builds/
#if defined(WIN32) && !defined(_DEBUG)
#pragma comment(linker, "/SUBSYSTEM:windows /ENTRY:mainCRTStartup")
#endif
int main() {
  // mejorar el Anti-Aliasing(MSAA)
  SetConfigFlags(FLAG_MSAA_4X_HINT);
  raylib::Window window(config::SCREEN_WIDTH, config::SCREEN_HEIGHT,
                        "raylib-cpp-demo");

  game::Game game;

  window.SetTargetFPS(75); // 60

  while (!window.ShouldClose()) {
    window.ClearBackground(raylib::Color::RayWhite());

    const float dt = GetFrameTime();
    if (IsKeyPressed(KEY_Q)) {
      std::printf("saliendo...\n");
      break;
    }

    game.update(dt);

    window.BeginDrawing();

    game.draw();

    window.EndDrawing();
  }

  return 0;
}

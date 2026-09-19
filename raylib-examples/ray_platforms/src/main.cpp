#include "raylib.h"
#include <array>

constexpr int SCREEN_WIDTH = 800;
constexpr int SCREEN_HEIGHT = 600;
constexpr int MAX_ITEMS = 3;

// un Enum para definir el eje
// en que la plataforma se mueve
enum class Axis { Horizontal, Vertical };

// plataformas/obstaculos fijos
struct Platform {
  Rectangle rect{};
  Color color{BLACK};

  void update(float dt) {}

  void draw() const {
    DrawRectangle(rect.x, rect.y, rect.width, rect.height, color);
  }
};

// plataformas/obstaculos moviles
struct MovingPlatform {
  Rectangle rect{};
  Color color{BLACK};
  float speed{10.0f};
  float direction{1.0f};
  float start_pos{0.0f};
  float end_pos{0.0f};
  Axis axis{Axis::Horizontal};

  MovingPlatform(Rectangle rect, Color color, float end_pos, float speed,
                 Axis axis)
      : rect(rect), color(color), end_pos(end_pos), speed(speed), axis(axis) {
    // posición de inicio segun el eje
    start_pos = (axis == Axis::Horizontal ? rect.x : rect.y);
  }

  void update(float dt) {

    switch (axis) {
    case Axis::Horizontal: {
      rect.x += direction * speed * dt;

      if (rect.x >= end_pos) {
        rect.x = end_pos;
        direction = -1.0f;
      } else if (rect.x <= start_pos) {
        rect.x = start_pos;
        direction = 1.0f;
      }
      break;
    }
    case Axis::Vertical: {
      rect.y += direction * speed * dt;

      if (rect.y >= end_pos) {
        rect.y = end_pos;
        direction = -1.0f;
      } else if (rect.y <= start_pos) {
        rect.y = start_pos;
        direction = 1.0f;
      }
      break;
    }
    }
  }

  void draw() const {
    DrawRectangle(rect.x, rect.y, rect.width, rect.height, color);
  }
};

//-----------------------------
// raylib 6.0 (Apr 23 2026)
//-----------------------------
int main() {
  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "raylib :: platforms");

  std::array<Platform, MAX_ITEMS> fixed_platforms{
      Platform{
          .rect = {320.0f, 420.0f, 100.0f, 30.0f},
          .color = BLUE,
      },
      Platform{
          .rect = {400.0f, 500.0f, 200.0f, 30.0f},
          .color = GREEN,
      },
      Platform{
          .rect = {150.0f, 550.0f, 300.0f, 30.0f},
          .color = BLUE,
      },
  };

  std::array<MovingPlatform, 2> moving_platforms{
      MovingPlatform{{200.0f, 330.0f, 200.0f, 30.0f},
                     PURPLE,
                     500.0f,
                     80.0f,
                     Axis::Horizontal},
      MovingPlatform{{400.0f, 100.0f, 100.0f, 30.0f},
                     YELLOW,
                     200.0f,
                     80.0f,
                     Axis::Vertical},
  };

  while (!WindowShouldClose()) {
    float dt = GetFrameTime();

    for (auto &mv_p : moving_platforms) {
      mv_p.update(dt);
    }

    BeginDrawing();

    ClearBackground(RAYWHITE);

    for (const auto &fx_pl : fixed_platforms) {
      fx_pl.draw();
    }

    for (const auto &mv_pl : moving_platforms) {
      mv_pl.draw();
    }

    EndDrawing();
  }

  CloseWindow();

  return 0;
}

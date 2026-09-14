#include "raylib.h"

#include <algorithm>
#include <cmath>
#include <iomanip>
#include <iostream>
#include <random>
#include <string>
#include <vector>

constexpr int SCREEN_WIDTH = 800;
constexpr int SCREEN_HEIGHT = 600;

constexpr float BALL_RADIUS = 10.0f;
constexpr int N = 25;                       // max 100
constexpr float PHYSICS_DT = 1.0f / 120.0f; // 60

class Ball {
public:
  float x{50.0f};
  float y{50.0f};
  float vx{300.0f}; // píxeles por segundo
  float vy{180.0f};
  float hit_timer{0.0f};

  // Ball() = default;
  // Ball() : x(50.0f), y(50.0f), vx(300.0f), vy(180.0f), hit_timer(0.0f) {}

  void update(float dt) {
    // calculamos el movimiento:
    // `posición = velocidad × tiempo`
    x += vx * dt;
    y += vy * dt;

    // rebote en los bordes
    // si la pelota sobrepasa algun borde (de la ventana)
    // se invierte la posición y se cambia la velocidad

    if (x < 0.0f) {
      x = -x;
      vx = -vx;
    } else if (x > SCREEN_WIDTH) {
      float exceso = x - SCREEN_WIDTH;
      x = SCREEN_WIDTH - exceso;
      vx = -vx;
    }

    if (y < 0.0f) {
      y = -y;
      vy = -vy;
    } else if (y > SCREEN_HEIGHT) {
      float exceso = y - SCREEN_HEIGHT;
      y = SCREEN_HEIGHT - exceso;
      vy = -vy;
    }

    // si la pelota ha chocado recientemente, se reduce
    // el contador de tiempo(hit_timer) para que el
    // destello amarillo desaparezca gradualmente
    if (hit_timer > 0.0f) {
      hit_timer -= dt;
    }
  }

  void draw() const {
    // `clamp` limita el valor (`hit_timer`) dentro de un
    // rango entre un mínimo(`0.0f`) y un máximo(`1.0f`)
    float t = std::clamp(hit_timer / 0.15f, 0.0f, 1.0f);

    Color color{255, static_cast<unsigned char>(t * 255.0f), 0, 255};

    DrawCircle(static_cast<int>(x), static_cast<int>(y), BALL_RADIUS, color);
  }
};

// detecta cuando dos pelotas se tocan y las hace rebotar
void handle_collisions(std::vector<Ball> &balls) {
  const float min_dist = BALL_RADIUS * 2.0f;
  size_t n = balls.size();

  for (size_t i = 0; i < n; ++i) {
    for (size_t j = i + 1; j < n; ++j) {
      // se obtiene la pelota 1
      // se obtiene la pelota 2
      // de esta forma se puede cambiar las
      // propiedades de `a` y `b` simultáneamente
      auto &a = balls[i];
      auto &b = balls[j];

      // calcular distancia entre sus centros
      // usando el teorema de Pitágoras

      float dx = b.x - a.x;
      float dy = b.y - a.y;

      // distancia al cuadrado (se puede usar sqrt)
      float dist2 = dx * dx + dy * dy;

      if (dist2 == 0.0f)
        continue;

      float dist = std::sqrt(dist2);

      // si la distancia es menor, hay colisión
      if (dist < min_dist) {
        // la "normal" es la dirección en la que
        // ocurre el choque entre las pelotas
        // al intercambiar velocidades completas (`x` y `y`),
        // simulamos un choque elástico (casi) perfecto
        float nx = dx / dist;
        float ny = dy / dist;

        // separar las pelotas para que no queden
        // superpuestas, empujarlas en direcciones opuestas
        float overlap = (min_dist - dist) * 0.5f;

        // se intercambian las velocidades

        a.x -= nx * overlap;
        a.y -= ny * overlap;

        b.x += nx * overlap;
        b.y += ny * overlap;

        // intercambiar velocidades
        // `std::swap` intercambia los valores de dos
        // variables sin crear copia
        // Es más eficiente que hacer
        // ````
        // float temp = a.vx;
        // a.vx = b.vx;
        // b.vx = temp;
        // ````
        std::swap(a.vx, b.vx);
        std::swap(a.vy, b.vy);

        // destello amarillo durante 150 ms
        a.hit_timer = 0.5f;
        b.hit_timer = 0.5f;
      }
    }
  }
}

// se dibujan texto y
// cualquier otra info
void draw_hud() {
  // dibujar los FPS
  // DrawFPS(10, 10);
  const std::string fps_text = "FPS: " + std::to_string(GetFPS());
  DrawText(fps_text.c_str(), 10, 10, 18, RED);
}

// crear las pelotas
std::vector<Ball> get_random_balls() {
  std::vector<Ball> balls;
  balls.reserve(N);

  std::mt19937 rng{std::random_device{}()};

  auto random = [&rng](float min, float max) {
    return std::uniform_real_distribution{min, max}(rng);
  };

  for (int i = 0; i < N; ++i) {
    balls.emplace_back(random(0, SCREEN_WIDTH), random(0, SCREEN_HEIGHT),
                       random(-300, 300), random(-300, 300));
  }

  return balls;
}

//-------------------------------
// raylib 6.0 (Apr 23 2026)
//-------------------------------
int main(void) {
  std::vector<Ball> balls = get_random_balls();

  // mejorar el Anti-Aliasing(MSAA)
  SetConfigFlags(FLAG_MSAA_4X_HINT);

  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "raylib::ball");

  SetTargetFPS(75);

  float accum{0.0f};

  // Main game loop
  while (!WindowShouldClose()) {
    float dt = GetFrameTime();

    // guarda el "tiempo sobrante" para
    // no perder frames
    accum += dt;

    //
    // input
    //

    //
    // Update
    //

    // sistema físico de paso fijo.
    // la física siempre avanza en pasos
    // iguales (según `PHYSICS_DT`)
    // independientemente de cuántos FPS
    // tenga el juego.
    while (accum >= PHYSICS_DT) {
      for (auto &ball : balls)
        ball.update(PHYSICS_DT);

      handle_collisions(balls);

      accum -= PHYSICS_DT;
    }

    //
    BeginDrawing();
    //

    ClearBackground(BLACK);
    for (const auto &ball : balls) {
      ball.draw();
    }

    draw_hud();

    //
    EndDrawing();
    //
  }

  // Close window and OpenGL context
  CloseWindow();

  return 0;
}

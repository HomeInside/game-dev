#include "game_over.hpp"

#include "raylib.h"

namespace states {
void GameOver::update(const float dt) {
  // lógica de GameOver
  (void)dt;
}

void GameOver::draw() const {
  ClearBackground(BLACK);

  constexpr char text[] = "GAME OVER";
  constexpr int font_size = 50;

  const int text_width = MeasureText(text, font_size);

  DrawText(text, (GetScreenWidth() - text_width) / 2, GetScreenHeight() / 2,
           font_size, RED);
}
} // namespace states

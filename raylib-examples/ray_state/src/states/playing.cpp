#include "playing.hpp"

#include "raylib.h"

namespace states {
void Playing::update(const float dt) {
  // lógica de Playing
  (void)dt;
}

void Playing::draw() const {
  ClearBackground(YELLOW);

  constexpr char text[] = "PLAYING";
  constexpr int font_size = 50;

  const int text_width = MeasureText(text, font_size);

  DrawText(text, (GetScreenWidth() - text_width) / 2, GetScreenHeight() / 2,
           font_size, RED);
}
} // namespace states

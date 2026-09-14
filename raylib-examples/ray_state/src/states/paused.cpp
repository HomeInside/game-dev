#include "paused.hpp"

#include "raylib.h"

namespace states {
void Paused::update(const float dt) {
  // lógica de Paused
  (void)dt;
}

void Paused::draw() const {
  ClearBackground(BLUE);

  constexpr char text[] = "PAUSED";

  const int font_size = 50;
  const int text_width = MeasureText(text, font_size);

  DrawText(text, (GetScreenWidth() - text_width) / 2, GetScreenHeight() / 2,
           font_size, RED);
}

} // namespace states

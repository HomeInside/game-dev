#include "main_menu.hpp"

#include "raylib.h"

namespace states {
void MainMenu::update(const float dt) {
  // lógica de MainMenu
  (void)dt;
}

void MainMenu::draw() const {
  ClearBackground(GREEN);

  constexpr char text[] = "MAIN MENU";

  const int font_size = 50;
  const int text_width = MeasureText(text, font_size);

  DrawText(text, (GetScreenWidth() - text_width) / 2, GetScreenHeight() / 2,
           font_size, RED);
}
} // namespace states

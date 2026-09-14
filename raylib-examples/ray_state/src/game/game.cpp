#include "game.hpp"

#include <iostream>
#include <variant>

namespace game {
// la implementación agrega operaciones
// comunes a todos los estados.

Game::Game() {
  // current_state = states::MainMenu{};
  // construye in-place
  current_state_.emplace<states::MainMenu>();
  bg_color_ = WHITE;
}

void Game::update(const float dt) {
  // cambiar de estado
  if (IsKeyPressed(KEY_ONE)) {
    current_state_ = states::MainMenu{};
    bg_color_ = BLACK;
  } else if (IsKeyPressed(KEY_TWO)) {
    current_state_ = states::Playing{};
    bg_color_ = BLACK;
  } else if (IsKeyPressed(KEY_THREE)) {
    current_state_ = states::Paused{};
    bg_color_ = BLACK;
  } else if (IsKeyPressed(KEY_FOUR)) {
    current_state_ = states::GameOver{};
    bg_color_ = WHITE;
  }

  // actualizar la escena activa
  std::visit([dt](auto &state) { state.update(dt); }, current_state_);
}

void Game::draw() const {
  // dibujar la escena activa
  std::visit([](const auto &state) { state.draw(); }, current_state_);
  // mostrar controles
  draw_hud();
}

void Game::draw_hud() const {
  constexpr std::string_view text =
      "1: Main Menu | 2: Playing | 3: Paused | 4: GameOver | Q: Salir";

  const int text_width = MeasureText(text.data(), 18);

  DrawText(text.data(), (GetScreenWidth() - text_width) - 20,
           GetScreenHeight() - 20, 18, bg_color_);

  // dibujar los FPS
  // DrawFPS(20, GetScreenHeight()-20);
  const std::string fps_text = std::to_string(GetFPS());
  DrawText(fps_text.c_str(), 20, (GetScreenHeight() - 20), 18, bg_color_);
}
} // namespace game

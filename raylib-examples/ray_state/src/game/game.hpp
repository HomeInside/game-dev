#pragma once

#include "raylib-cpp.hpp"
#include "states/game_over.hpp"
#include "states/main_menu.hpp"
#include "states/paused.hpp"
#include "states/playing.hpp"

#include <variant>

namespace game {
// se crea un `std::variant` que nos permite
// mantener un solo estado del juego al tiempo.
//
// cada variante contiene un struct/class
// que se encarga de implementar el
// comportamiento específico de cada
// estado.
// Esto garantiza que el juego se encuentra
// en un único estado a la vez.
using GameState = std::variant<states::MainMenu, states::Playing,
                               states::Paused, states::GameOver>;

// implementacion principal del juego
class Game {
public:
  Game();

  void update(float dt);
  void draw() const;
  void draw_hud() const;

private:
  GameState current_state_;
  Color bg_color_;
};
} // namespace game

#pragma once

namespace states {
class MainMenu {
public:
  // se mantiene como clase para ejemplificar
  // algunos conceptos
  // el compilador genera el constructor
  // automágicamente
  MainMenu() = default;

  void update(float dt);
  void draw() const;
};
} // namespace states

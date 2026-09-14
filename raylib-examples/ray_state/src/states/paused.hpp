#pragma once

namespace states {
struct Paused {
  void update(float dt);
  void draw() const;
};
} // namespace states

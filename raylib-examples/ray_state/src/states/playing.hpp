#pragma once

namespace states {
struct Playing {
  void update(float dt);
  void draw() const;
};
} // namespace states

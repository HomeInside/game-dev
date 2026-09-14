#include "raylib.h"

#include <algorithm>
#include <string>

// un struct para el dinosaurio
// claro que sí, puede ser una clase
struct Dino {
  Texture2D image{};
  float w{};
  float h{};
  Vector2 position{};
  Vector2 speed{};
  Rectangle hitbox{};

  // constructor
  /*
  explicit Dino(const Image& source)
  {
      image = LoadTextureFromImage(source);
      w = static_cast<float>(source.width);
      h = static_cast<float>(source.height);
      position = Vector2{0.0f, 0.0f};
      speed = Vector2{0.0f, 0.0f};
      hitbox = Rectangle{0.0f, 0.0f, w, h};
  }
  */

  // usando lista de inicializacion(WTF!)
  explicit Dino(const Image &source)
      : image(LoadTextureFromImage(source)),
        w(static_cast<float>(source.width)),
        h(static_cast<float>(source.height)), position({0.0f, 0.0f}),
        speed({0.0f, 0.0f}), hitbox({0.0f, 0.0f, w, h}) {}

  // destructor
  ~Dino() {
    // Once image has been converted to texture
    // and uploaded to VRAM, it can be unloaded
    // from RAM
    UnloadTexture(image);
  }

  Dino(const Dino &) = delete;
  Dino &operator=(const Dino &) = delete;

  // se devuelve la instancia
  // sincronizada en `set_in_window`
  Rectangle get_rect() const { return hitbox; }

  // se dibuja el rect al rededor del dino
  void draw_hitbox() const { DrawRectangleLinesEx(get_rect(), 3, RED); }

  // actualiza la posicion y el hitbox
  // mantiene el dinosaurio dentro de
  // la ventana principal
  void update(float dt) {
    // Actualizar posición con velocidad
    position.x += speed.x * dt;
    position.y += speed.y * dt;

    // Mantener dentro de la ventana
    position.x = std::clamp(position.x, 0.0f, GetScreenWidth() - w);
    position.y = std::clamp(position.y, 0.0f, GetScreenHeight() - h);

    // Sincronizar hitbox
    hitbox.x = position.x;
    hitbox.y = position.y;
  }

  void draw() const { DrawTextureV(image, position, WHITE); }
};

// un struct para el obstaculo
struct Obstacle {
  Rectangle rect{};
  Color color{BLACK};

  // constructor
  /*
  explicit Obstacle(const Rectangle& fig)
  {
      rect = fig;
      color = BLACK;
  }
  */
  // explicit Obstacle(const Rectangle& fig) : rect(fig) {}

  // no hay destructor aqui

  void draw() const { DrawRectangleLinesEx(rect, 3, color); }

  // se devuelve el rectangulo
  Rectangle get_rect() const { return rect; }

  // se dibuja el rect del obstaculo
  void draw_hitbox() const {
    // return Rectangle{position.x, position.y, float(image.width),
    // float(image.height)};
    DrawRectangleLinesEx(rect, 3, GREEN);
  }
};

// se dibujan texto y
// cualquier otra info
void draw_hud() {
  // dibujar los FPS
  // DrawFPS(10, 10);
  const std::string fps_text = "FPS: " + std::to_string(GetFPS());
  DrawText(fps_text.c_str(), 10, 10, 18, RED);
}

//-------------------------------
// raylib 6.0 (Apr 23 2026)
//-------------------------------
int main(void) {
  constexpr int SCREEN_WIDTH = 800;
  constexpr int SCREEN_HEIGHT = 600;

  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "raylib::hitbox");

  // NOTE: Textures MUST be loaded after
  // Window initialization (OpenGL context is required)
  Image image = LoadImage("dino.png");

  // instancia del dino
  Dino dino{image};

  // Rectangle rect = Rectangle{200, 200, 200, 175};
  // Obstacle obstacle{rect};
  //
  //  instancia del obstaculo
  Obstacle obstacle{.rect = {200, 200, 200, 175}};

  int speed = 75;

  SetTargetFPS(60);

  // Main game loop
  while (!WindowShouldClose()) {
    float dt = GetFrameTime();
    dino.speed = Vector2{0.0f, 0.0f};

    //
    // input/update
    //

    if (IsKeyDown(KEY_RIGHT)) {
      dino.speed.x += speed;
    }

    if (IsKeyDown(KEY_LEFT)) {
      dino.speed.x -= speed;
    }

    if (IsKeyDown(KEY_UP)) {
      dino.speed.y -= speed;
    }

    if (IsKeyDown(KEY_DOWN)) {
      dino.speed.y += speed;
    }

    //
    // Update
    //

    dino.update(dt);

    bool is_colliding =
        CheckCollisionRecs(dino.get_rect(), obstacle.get_rect());

    //
    // Draw
    //
    BeginDrawing();
    //

    ClearBackground(RAYWHITE);

    dino.draw();

    if (is_colliding) {
      dino.draw_hitbox();
      obstacle.draw_hitbox();
      DrawText("objetos colisionando!", 10, 25, 22, RED);
    } else {
      obstacle.draw();
    }

    draw_hud();

    //
    // End Draw
    //
    EndDrawing();
    //
  }

  // Close window and OpenGL context
  CloseWindow();

  return 0;
}

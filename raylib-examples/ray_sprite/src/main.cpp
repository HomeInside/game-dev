#include "raylib.h"

constexpr int SCREEN_WIDTH = 800;
constexpr int SCREEN_HEIGHT = 600;

struct Player {
  Texture2D image{};
  int width{0};       // ancho de cada frame la imagen
  int height{0};      // alto de cada frame de la imagen
  int totalFrames{0}; // frames de la imagen
  int currentFrame{0};
  float speed{0.16f};
  float timer{0.0f};
  Vector2 position{0, 0};
  bool flip{false};

  Player(Texture2D sprite, int width, int numFrames, float speed, Vector2 pos)
      : image(sprite), width(sprite.width / numFrames), height(sprite.height),
        totalFrames(numFrames), speed(speed), position(pos) {}

  void update(float dt, bool use_flip = false) {
    timer += dt;

    if (timer >= speed) {
      timer = 0.0f;
      currentFrame++;
      if (currentFrame >= totalFrames)
        currentFrame = 0;
    }

    flip = use_flip;
  }

  void draw() {

    // rectángulo del frame actual en el sprite sheet
    Rectangle sourceRec = {static_cast<float>(currentFrame * width), 0.0f,
                           static_cast<float>(width),
                           static_cast<float>(height)};

    // con `width` negativo, RayLib lee el rectángulo de derecha a izquierda,
    // volteando la imagen, sin embargo asegurate de que `.x` debe seguir
    // siendo el borde izquierdo del frame original, no el derecho.
    if (flip) {
      sourceRec.x =
          static_cast<float>((currentFrame + 1) * width); // borde derecho
      sourceRec.width = -static_cast<float>(width);
    }

    // destino del sprite en pantalla
    Rectangle destRec = {position.x, position.y, static_cast<float>(width),
                         static_cast<float>(height)};

    // origen de rotación en el centro del sprite en la pantalla
    Vector2 origin = {width / 2.0f, height / 2.0f};
    DrawTexturePro(image, sourceRec, destRec, origin, 0.0f, WHITE);
  }
};

//-----------------------------
// raylib 6.0 (Apr 23 2026)
//-----------------------------
int main() {
  InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "raylib :: sprite");
  int ScreenWidth = static_cast<int>(GetScreenWidth() / 2);
  int ScreenHeight = static_cast<int>(GetScreenHeight() / 2);

  // cargar el sprite sheet idle
  Texture2D sprite_idle = LoadTexture("ray_sprite/assets/male_hero-idle.png");
  Player player_idle{sprite_idle, 128, 10, 0.16f, {100, ScreenHeight}};

  // cargar el sprite sheet walk
  Texture2D sprite_walk = LoadTexture("ray_sprite/assets/male_hero-walk.png");
  Player player_walk{sprite_walk, 128, 10, 0.13f, {200, ScreenHeight}};

  // cargar el sprite sheet run
  Texture2D sprite_run = LoadTexture("ray_sprite/assets/male_hero-run.png");
  Player player_run{sprite_run, 128, 10, 0.05f, {300, ScreenHeight}};

  // cargar el sprite sheet jump
  Texture2D sprite_jump = LoadTexture("ray_sprite/assets/male_hero-jump.png");
  Player player_jump{sprite_jump, 128, 6, 0.14f, {400, ScreenHeight}};

  // cargar el sprite sheet fall
  Texture2D sprite_fall = LoadTexture("ray_sprite/assets/male_hero-fall.png");
  Player player_fall{sprite_fall, 128, 4, 0.16f, {500, ScreenHeight}};

  SetTargetFPS(60);

  // podemos girar el sprite
  bool flip = false;

  while (!WindowShouldClose()) {
    float dt = GetFrameTime();

    if (IsKeyPressed(KEY_ONE)) {
      // gira a la izquierda
      flip = true;
    } else if (IsKeyPressed(KEY_TWO)) {
      // gira a la derecha
      flip = false;
    }

    player_idle.update(dt, flip);
    player_walk.update(dt, flip);
    player_run.update(dt, flip);
    player_jump.update(dt, flip);
    player_fall.update(dt, flip);

    BeginDrawing();

    ClearBackground(RAYWHITE);

    player_idle.draw();
    player_walk.draw();
    player_run.draw();
    player_jump.draw();
    player_fall.draw();

    DrawText(TextFormat("usa las teclas [1] y [2] para girar el sprite"), 10,
             10, 20, DARKGRAY);

    EndDrawing();
  }

  UnloadTexture(sprite_idle);
  UnloadTexture(sprite_run);
  UnloadTexture(sprite_run);
  UnloadTexture(sprite_jump);
  UnloadTexture(sprite_fall);

  CloseWindow();

  return 0;
}

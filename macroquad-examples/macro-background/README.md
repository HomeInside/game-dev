# Render Target, Fondos y mosaicos
es un búfer de memoria donde se dibujan gráficos antes de la visualización

```
1. Cargar textura pequeña(un patrón, ej: `Blue.png`)
2. Pre-renderizar el mosaico 
   ├─ Crear RenderTarget (invisible)
   ├─ Crear una camara para el RenderTarget
   ├─ Dibujar el mosaico (N llamadas)
   └─ Devolver textura renderizada
3. Bucle principal
   └─ Dibujar textura pre-renderizada
```


## Entrada: Textura pequeña (64x64)

```
┌──────────┐
│  Blue    │
│  tile    │
└──────────┘
```


## Proceso: Pre-renderizado en RenderTarget

```
┌─────────────────────────────────────────┐
│ Blue │ Blue │ Blue │ Blue │ Blue │ Blue │  ← Fila 0
├──────┼──────┼──────┼──────┼──────┼──────┤
│ Blue │ Blue │ Blue │ Blue │ Blue │ Blue │  ← Fila 1
├──────┼──────┼──────┼──────┼──────┼──────┤
│ Blue │ Blue │ Blue │ Blue │ Blue │ Blue │  ← Fila 2
├──────┼──────┼──────┼──────┼──────┼──────┤
│ Blue │ Blue │ Blue │ Blue │ Blue │ Blue │  ← Fila 3
└─────────────────────────────────────────┘
```
- Cada "Blue" es una copia de la textura original
- Se crea un mosaico completo que cubre toda la ventana


## Salida: Una única textura

```
┌───────────────────────────────────────┐
│  Mosaico completo pre-renderizado     │
│  (800x600 píxeles)                    │
└───────────────────────────────────────┘
```

- Una sola imagen que contiene el mosaico completo
- Se dibuja en pantalla


## Casos de uso

1. **Fondos de juegos**: Mosaicos de texturas (césped, agua, tierra)
2. **UI estática**: Fondos de menús o paneles
3. **Patrones decorativos**: Texturas repetidas en aplicaciones
4. **Optimización de rendimiento**: Reducir drásticamente las llamadas de dibujo


## Limitaciones y consideraciones

### Ventanas redimensionables

- Si la ventana cambia de tamaño, el RenderTarget debe regenerarse
- Solución: Detectar cambios de tamaño y llamar a `create_background` nuevamente

### Uso de memoria

- El RenderTarget ocupa: `ancho × alto × 4 bytes` (RGBA)
- Para 800x600: 1.92 MB
- Para pantallas 4K: ~32 MB

### Recomendaciones

- considera utilizar patrones de imagenes pequeños


## Referencias

- [render_target (macroquad)](https://docs.rs/macroquad/latest/macroquad/texture/fn.render_target.html)
- [set_default_camera (macroquad)](https://docs.rs/macroquad/latest/macroquad/camera/fn.set_default_camera.html)
- [Starfield shader (macroquad)](https://mq.agical.se/ch9-starfield-shader.html)
- [Render Target (Unity)](https://unity.com/es/glossary/render-target)
- [Render Target (monogame.net)](https://docs.monogame.net/articles/getting_to_know/whatis/graphics/WhatIs_Render_Target.html)

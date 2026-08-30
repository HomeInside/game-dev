## Efecto Parallax

El [parallax](https://es.wikipedia.org/wiki/Paralaje) es una técnica visual que simula **profundidad** moviendo capas de fondo a diferentes velocidades. Las capas lejanas se mueven más lento que las cercanas, creando una sensación de inmersión y tridimensionalidad.


## Tipos de desplazamiento

### Parallax Horizontal
- el movimiento ocurre de **izquierda a derecha** (o viceversa).
- ideal para: juegos de plataformas, carreras, desplazamiento lateral.
- las capas se superponen en el eje **X**.

Ejemplo de velocidades:
- cielo/nubes → `0.1x` (casi quieto)
- montañas → `0.3x` 
- bosque → `0.6x`
- primer plano → `1.0x` (velocidad real)

## Parallax Vertical
- el movimiento ocurre de **arriba hacia abajo** (o viceversa).
- ideal para: juegos de disparo vertical, ascensores, caídas.
- las capas se superponen en el eje **Y**.

Ejemplo de velocidades:
- fondo estrellado → `0.2x`
- nubes → `0.5x`
- edificios → `0.8x`
- primer plano → `1.0x`


## Técnicas de implementación

### scroll infinito
- la imagen se mueve continuamente sin necesidad de intervención.
- útil para: fondos animados, pantallas de título, efectos ambientales.

### desplazamiento controlado
- el movimiento depende de la posición del jugador/cámara.
- útil para: juegos interactivos donde el fondo responde a la acción.


## truco del borde infinito

para lograr un **scroll infinito** sin cortes:

- dos ó mas copias: Se dibuja la imagen principal y una copia a su lado (o debajo).
- reinicio modular: Cuando la imagen sale de la pantalla, se "resetea" a la posición opuesta.
- solapamiento: Un pequeño solapamiento (1-2 píxeles) evita líneas visibles entre copias.

### en horizontal

```
[IMG] [COPIA]    <- las dos imágenes lado a lado
   ↓      ↓
[COPIA] [IMG]    <- después del desplazamiento
```

### en vertical
```
 ↓        las dos imágenes una debajo de la otra
[IMG]

[COPIA]  

  ↓       después del desplazamiento

[COPIA]

[IMG]
```


## assets 

- horizontal parallax
[@oleekconder](https://oleekconder.itch.io/) - [hand-painted-parallax-background](https://oleekconder.itch.io/hand-painted-parallax-background)

- vertical parallax 
[@TheClicketyBoom](https://opengameart.org/users/theclicketyboom) - [Nebula](https://opengameart.org/content/3-layer-parallax-star-and-nebula-field)


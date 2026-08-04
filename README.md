# plataformas en Macroquad

## Descripción

Este proyecto es un pequeño prototipo de juego de plataformas 2D.

El objetivo principal es construir, paso a paso, las bases de un juego donde un jugador pueda moverse dentro de un mundo con gravedad, saltar, interactuar con plataformas y tener colisiones sólidas.

## Mundo del juego

El escenario está formado por:

* Un jugador.
* Varias plataformas fijas.
* Los límites de la ventana como frontera del mundo.

Las plataformas funcionan como superficies sólidas: el jugador no puede atravesarlas y pueden ser usadas como apoyo para moverse y saltar.

## Jugador

El jugador cuenta con:

* Movimiento horizontal.
* Caída por gravedad.
* Capacidad de salto.
* Detección de contacto con superficies.
* Límites para evitar salir del mundo.

El jugador tiene una posición dentro del escenario y un área de detección que representa su espacio físico dentro del juego.

## Gravedad

El juego utiliza un sistema de gravedad que afecta constantemente al jugador.

Cuando el jugador está en el aire:

* Su velocidad vertical aumenta hacia abajo.
* La posición cambia con el paso del tiempo.
* Al tocar una superficie, la caída se detiene.

La gravedad sigue actuando incluso cuando el jugador está sobre una plataforma; la plataforma simplemente evita que continúe cayendo.

## Saltos

El salto funciona aplicando una fuerza inicial hacia arriba.

El movimiento del salto sigue una trayectoria similar a una parábola:

1. El jugador inicia el salto con una velocidad hacia arriba.
2. La gravedad reduce esa velocidad.
3. El jugador alcanza una altura máxima.
4. La gravedad hace que vuelva a caer.

La altura del salto depende de la fuerza inicial del salto y de la gravedad del mundo.
```math
h=\frac{v^2}{2g}
```
donde:

* `v` = velocidad inicial del salto (`JUMP_SPEED`)
* `g` = gravedad (`GRAVITY`)

## Plataformas

Actualmente existen varias plataformas fijas dentro del escenario.

Las plataformas permiten:

* Aterrizar encima.
* Chocar por la parte inferior.
* Bloquear el movimiento lateral.

Cada plataforma tiene un área visible y un área de colisión para facilitar la prueba y depuración del comportamiento.

## Colisiones

El sistema de colisiones busca que las plataformas se comporten como objetos sólidos.

Actualmente permite:

* Evitar atravesar plataformas por arriba.
* Evitar atravesar plataformas por abajo.
* Evitar atravesar plataformas por los lados.
* Permanecer apoyado sobre una plataforma.
* Usar una plataforma como punto de salto.

El movimiento y las colisiones se revisan por separado en los ejes horizontal y vertical para controlar mejor el comportamiento del jugador.

## Estado actual

Actualmente el prototipo cuenta con:

* Movimiento básico.
* Gravedad.
* Saltos.
* Piso del mundo.
* Plataformas sólidas.
* Detección de colisiones.
* Visualización de áreas de colisión para pruebas.

## Próximos pasos

Algunas mejoras futuras:

* Mejorar las colisiones en esquinas.
* Ajustar la sensación del salto.
* Agregar más elementos del mundo.
* Incorporar enemigos u objetos interactivos.
* Crear niveles más completos.
* Mejorar animaciones y controles.

## Objetivo del proyecto

Este proyecto busca entender y construir desde cero las bases de un juego de plataformas:

* cómo se mueve un personaje;
* cómo funciona la gravedad;
* cómo se producen los saltos;
* cómo interactúan los objetos dentro de un mundo virtual.

La prioridad es comprender cada sistema antes de agregar complejidad.

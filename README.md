# game-dev

Este repositorio reúne varios ejercicios de temas relevantes para el desarrollo de videojuegos en 2d.


## Ejercicios propuestos

Los ejemplos pueden estar en un lenguaje u otro, pero los conceptos son transversales.

### En Rust
Los ejercicios en [Rust](https://rust-lang.org/) están principalmente desarrollados con:

- [ggez](https://ggez.rs/)
- [Macroquad](https://macroquad.rs/)

así que contienen el prefijo `macro-xxx` ó `ggez-xxx`, para diferenciarlos.


### En C++
Los ejercicios en [C++ (C++ 20 en adelante)](https://isocpp.org/) están principalmente desarrollados con:

- [raylib](https://www.raylib.com/)


así que contienen el prefijo `ray-xxx`, para diferenciarlos.


## Como contribuir

- no hay mejor lugar que la documentación que ofrece la librería ó framework de tu elección.
- [Estados y Organización](https://github.com/HomeInside/game-dev/tree/master/macroquad-examples/macro-state), el ejemplo de **macro-state**: un buen lugar para empezar.
- crea un [fork](https://github.com/HomeInside/game-dev/fork) ó [clona el repo](https://docs.github.com/es/repositories/creating-and-managing-repositories/cloning-a-repository).
- explica un tema relevante (revisa los ya expuestos aquí), creando un ejercicio en el lenguaje de tu preferencia.
- documenta.
- comparte!.


## Editores de mapas

- [Tiled](https://www.mapeditor.org/)
- [LDtk](https://ldtk.io/)


## Otros lenguajes
Si el lenguaje que usas no esta aquí, echale un vistazo a:

- C/C++
	- [SFML 3.x](https://www.sfml-dev.org/)
	- [Cocos2d-x](https://www.cocos.com/en/cocos2d-x)
	- raylib tiene bindings [para muchos lenguajes](https://github.com/raysan5/raylib/blob/master/BINDINGS.md)
	- [SDL](https://www.libsdl.org/)
	- [Oxygine](https://oxygine.org/)
	- [Allegro](https://liballeg.org/)

	[... y muchísimos más...](https://gamefromscratch.com/c-c-game-engines-in-2025/)

- Go
	- [Ebitengine](https://ebitengine.org/)
	- [Pixel 2](https://github.com/gopxl/pixel)
	- [Kaiju Engine](https://kaijuengine.com/)
	- [Engo](https://engoengine.github.io/)
	[... y muchísimos más...](https://awesome-go.com/game-development/)

- Javascript

	- [Phaser](https://phaser.io/)
	- [pixijs](https://pixijs.com/)
	- [melonJS](https://melonjs.org/)

	[... y muchísimos más...](https://gamefromscratch.com/javascript-typescript-game-engines-in-2025/)

- Haxe
	- [HaxeFlixel](https://haxeflixel.com/)
	- [Heaps](https://heaps.io/index.html)

- Lua
	- [LÖVE](https://love2d.org/)
	- [Solar2D](https://solar2d.com/)
	- [Defold](https://defold.com/)

- Python
	- [Pygame-ce](https://pyga.me/)
	- [Python Arcade](https://api.arcade.academy/en/latest/#)
	- [Pyglet](https://pyglet.org)

	[... y muchísimos más...](https://gamefromscratch.com/python-game-engines-in-2025/)


## Recursos para videojuegos

- [magictools](https://github.com/ellisonleao/magictools)
- [Kenney](https://kenney.nl/)
- [Itch](https://itch.io/)
- [craftpix](https://craftpix.net/)
- [graphicburger](https://graphicburger.com/)
- [game-icons](https://game-icons.net/)
- [piskelapp](https://www.piskelapp.com/)
- [libresprite](https://libresprite.github.io/)
- [aseprite](https://www.aseprite.org/)
- [gamefromscratch](https://gamefromscratch.com/news/)

## Libros y más documentación

- [2D Game Development: From Zero To Hero](https://therealpenaz91.itch.io/2dgd-f0th)
- [Awesome Game Engine Development](https://github.com/stevinz/awesome-game-engine-dev)
- [Custom Game Engines](https://github.com/raysan5/custom_game_engines)
- [gamefromscratch](https://gamefromscratch.com/)
- [Game Engine Black Book DOOM](https://fabiensanglard.net/gebbdoom/)
- [The Level Design Book](https://book.leveldesignbook.com/)
- [3D Game Shaders For Beginners](https://lettier.github.io/3d-game-shaders-for-beginners/index.html)


## Acerca de este repositorio(Monorepo)

Este repositorio es un monorepo con repositorios hijos en subcarpetas **sin perder el historial**, usando `git subtree`. Por lo que cada ejercicio puede ser transportado, compilado y ejecutado de forma independiente.


### Requisitos

 - [Git](https://git-scm.com/) **2.55.x** ó superior
 - [Just (opcional pero recomendado)](https://github.com/casey/just) **1.47.x** ó superior
 
- Repositorios hijos accesibles localmente (o remotes fetchables)

### Instalar subtree (Fedora 44)

```bash
sudo dnf install git-subtree
```

### Agregar un repo existente

```bash
# fijar el nombre de la rama principal
git config --global init.defaultBranch master
```

```bash
cd rs-work/
#
git remote add REPO_NAME /path/to/repository
git fetch REPO_NAME
git subtree add --prefix=REPO_NAME REPO_NAME master
#
git remote -v
git push --set-upstream origin master
git push origin master --tags
#
# aplicar cambios
#
git subtree pull --prefix=REPO_NAME REPO_NAME master
#
# ó de la forma con `--squash` si se prefiere
# un único commit por actualización
git subtree pull --prefix=REPO_NAME REPO_NAME master --squash
#
git push origin master --tags
```


## Licencia

El repositorio **game-dev** está sujeto a cualquiera de las siguientes licencias, a tu elección:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])

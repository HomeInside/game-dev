# game-dev

Este repositorio reúne varios ejercicios de temas relevantes para el desarrollo de videojuegos en 2d.


## Ejercicios propuestos

### En Rust
Los ejercicios en [Rust](https://rust-lang.org/) están principalmente desarrollados con:

- [ggez](https://ggez.rs/)
- [Macroquad](https://macroquad.rs/)

así que contienen el prefijo `macro-xxx` ó `ggez-xxx`, para diferenciarlos.


### C++
Los ejercicios en [C++ (C++ 20 en adelante)](https://isocpp.org/) están principalmente desarrollados con:

- [raylib](https://www.raylib.com/)
- [SFML 3.x](https://www.sfml-dev.org/)

así que contienen el prefijo `raylib-xxx` ó `sfml-xxx`, para diferenciarlos.

muchos ejemplos, pueden estar en un lenguaje u otro, pero conceptos son transversales.


## Como contribuir

- [Estados y Organización](https://github.com/HomeInside/game-dev/tree/master/macro-state), el ejemplo de **macro-state**: un buen lugar para empezar.
- crea un [fork](https://github.com/HomeInside/game-dev/fork) ó [clona el repo](https://docs.github.com/es/repositories/creating-and-managing-repositories/cloning-a-repository).
- explica un tema relevante (revisa los ya expuestos aquí), creando un ejercicio en el lenguaje de tu preferencia.
- documenta.
- comparte!.


## Monorepo

Este repositorio es un monorepo con repositorios hijos en subcarpetas **sin perder el historial**, usando `git subtree`. Por lo que cada ejercicio puede ser transportado, compilado y ejecutado de forma independiente.


## Requisitos

 - [Git](https://git-scm.com/) **2.55.x** ó superior
 - [just](https://github.com/casey/just) **1.47.x** ó superior
 
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

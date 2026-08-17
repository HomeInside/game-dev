# rs-work (Monorepo)

Este repositorio reúne varios ejercicios en [Rust](https://www.rust-lang.org), con varios repositorios hijos en subcarpetas **sin perder el historial**,
usando `git subtree`.

## Requisitos

 - [Git](https://git-scm.com/) **2.55.x** ó superior
 - [just](https://github.com/casey/just) **1.47.x** ó superior
 


- Repositorios hijos accesibles localmente (o remotes fetchables)

### Instalar subtree (Fedora 44)

```bash
sudo dnf install git-subtree
```

## Agregar un repo existente

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

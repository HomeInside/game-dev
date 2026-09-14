# Raylib examples

## Requirements

- [raylib](https://www.raylib.com/) **6.0**
- [raylib-cpp](https://robloach.github.io/raylib-cpp/) **6.0.3**
- [GCC](https://gcc.gnu.org/) **15.2.x** and above
- [Cmake](https://cmake.org/download/) **3.28.x** and above

## Recommended tools

- [just](https://github.com/casey/just) **1.47.x** and above
- [clang-format](https://clang.llvm.org/docs/ClangFormat.html)
- [clang-tidy](https://clang.llvm.org/extra/clang-tidy/)
- [clangd (lsp)](https://clangd.llvm.org/)

## Getting Started

### Clone the repo and make it yours:

```bash
$ git clone https://github.com/HomeInside/game-dev.git
```

```bash
$ cd ray_demo
```

## Windows static & dynamic

## mingw64

- raylib-6.0_win32_mingw-w64.zip
- https://github.com/raysan5/raylib/releases

## Windows from repo

```sh
# inside ray_demo folder
$ git clone --depth 1 https://github.com/raysan5/raylib.git raylib
```

### raylib-cpp

```sh
# inside ray_demo folder
git clone --depth 1 https://github.com/RobLoach/raylib-cpp.git
```

## Configure

```bash
$ just configure
```

### Build

```bash
$ just build
```

### Run

```bash
$ just run
```

## License

**ray_demo** is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])

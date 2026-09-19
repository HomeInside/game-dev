# Just task runner

set dotenv-load := false
just_home := justfile_directory()

# for Windows
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]
os := os()

PROJECT_NAME := "ray_sprite"

CMAKE_BUILD_DIR := if os == "windows" {
  "cmake-build-debug"
} else {
  "build"
}

# for Windows
# import 'test/justfile'

[doc("📋 Show all recipes")]
[group("Help")]
help:
    @just --list

[group("Help")]
default: help

[doc("🎨 Format code")]
[group("Development")]
[unix]
fmt:
    # clang-format -i include/*.hpp src/*.cpp
    clang-format -i {{PROJECT_NAME}}/src/*.cpp

[doc("🔍 Check formatting (CI mode)")]
[group("Development")]
[unix]
fmt-check:
    # clang-format -i {{PROJECT_NAME}}/src/*.cpp {{PROJECT_NAME}}/include/*.hpp --Werror
    clang-format -i {{PROJECT_NAME}}/src/*.cpp --Werror

[group("Maintenance")]
[windows]
clean:
    echo "clean..."

[group("Maintenance")]
[unix]
clean:
    rm -rf build/*

clean-all: clean
    rm -rf cmake-build-debug/*
    rm -rf cmake-build-release/*
    rm -rf build/*

[group("Maintenance")]
gitc:
    git fsck && git gc --prune=now --aggressive && git count-objects -vH

[unix]
configure:
    # cmake --preset debug
    # sin presets
    cmake -B build -S . -DCMAKE_BUILD_TYPE=Debug

[unix]
release:
    cmake -B build -DCMAKE_BUILD_TYPE=Release

[unix]
build: fmt
    # cmake --build --preset debug -j
    # cmake --build --preset debug --target {{ PROJECT_NAME }} -j
    # sin presets
    cmake --build build

[unix]
run:
    ./{{ CMAKE_BUILD_DIR }}/{{ PROJECT_NAME }}

[unix]
dev: build run

home:
    echo {{just_home}}

[unix]
w:
    gcc --version;
    g++ --version;
    cmake --version;
    ninja --version;

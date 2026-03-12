#!/bin/sh
set -e

debug=false
while getopts d: flag
do
    case "${flag}" in
        d) debug=${OPTARG};;
    esac
done

build_type=Release
if [ "$debug" = "true" ]; then
  build_type=Debug
fi

function sdl3() {
    cd SDL
    cmake -S . -B build -DSDL_STATIC=ON -DCMAKE_BUILD_TYPE=$build_type
    cmake --build build --config $build_type
    sudo cmake --install build --prefix /usr/local
    cd ..
}

function glm() {
    cd glm
    cmake -DGLM_BUILD_TESTS=OFF -DBUILD_SHARED_LIBS=OFF -B build .
    cmake --build build -- all
    sudo cmake --build build -- install
    cd ..
}

function miniaudio() {
    cd miniaudio
    cmake -B build . -DCMAKE_BUILD_TYPE=$build_type
    cmake --build build --config $build_type
    sudo cmake --install build
    cd ..
}

function zlib() {
    cd zlib
    cmake -S . -B build -DBUILD_SHARED_LIBS=OFF -DCMAKE_BUILD_TYPE=$build_type
    cmake --build build --config $build_type
    sudo cmake --install build --prefix /usr/local
    cd ..
}

function libpng() {
    cd libpng
    cmake -S . -B build -DPNG_SHARED=OFF -DPNG_STATIC=ON -DPNG_BUILD_ZLIB=OFF -DCMAKE_BUILD_TYPE=$build_type
    cmake --build build --config $build_type
    sudo cmake --install build --prefix /usr/local
    cd ..
}

function vulkan() {
  sudo pacman -S vulkan-tools vulkan-devel spirv-tools
}

function wayland() {
  sudo pacman -S --needed wayland wayland-protocols libxkbcommon-x11 libxkbcommon
}

sudo pacman -Suy

sdl
glm
miniaudio
zlib
libpng
vulkan
wayland

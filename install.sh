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
    cmake -S . -B build -DPNG_SHARED=OFF -DPNG_STATIC=ON -DCMAKE_BUILD_TYPE=$build_type
    cmake --build build --config $build_type
    sudo cmake --install build --prefix /usr/local
    cd ..
}

function llvm() {
    cd llvm-project/llvm
    cmake -S . -B build \
        -DCMAKE_BUILD_TYPE=$build_type \
        -DCMAKE_INSTALL_LIBDIR=lib \
        -DBUILD_SHARED_LIBS=OFF \
        -DLLVM_ENABLE_PROJECTS= \
        -DLLVM_ENABLE_RUNTIMES= \
        -DLLVM_TARGETS_TO_BUILD=Native \
        -DLLVM_APPEND_VC_REV=OFF \
        -DLLVM_BUILD_TOOLS=OFF \
        -DLLVM_INCLUDE_TOOLS=OFF \
        -DLLVM_BUILD_UTILS=OFF \
        -DLLVM_INCLUDE_UTILS=OFF \
        -DLLVM_BUILD_RUNTIME=OFF \
        -DLLVM_INCLUDE_RUNTIMES=OFF \
        -DLLVM_BUILD_EXAMPLES=OFF \
        -DLLVM_INCLUDE_EXAMPLES=OFF \
        -DLLVM_BUILD_TESTS=OFF \
        -DLLVM_INCLUDE_TESTS=OFF \
        -DLLVM_BUILD_BENCHMARKS=OFF \
        -DLLVM_INCLUDE_BENCHMARKS=OFF \
        -DLLVM_BUILD_DOCS=OFF \
        -DLLVM_INCLUDE_DOCS=OFF \
        -DLLVM_ENABLE_BINDINGS=OFF \
        -DLLVM_ENABLE_OCAMLDOC=OFF \
        -DLLVM_ENABLE_TELEMETRY=OFF \
        -DLLVM_ENABLE_PLUGINS=OFF \
        -DLLVM_BUILD_LLVM_DYLIB=OFF \
        -DLLVM_BUILD_LLVM_C_DYLIB=OFF \
        -DLLVM_ENABLE_ZLIB=OFF \
        -DLLVM_ENABLE_ZSTD=OFF \
        -DLLVM_ENABLE_LIBXML2=OFF \
        -DLLVM_ENABLE_LIBEDIT=OFF \
        -DLLVM_ENABLE_LIBPFM=OFF \
        -DLLVM_ENABLE_CURL=OFF \
        -DLLVM_ENABLE_HTTPLIB=OFF \
        -DLLVM_ENABLE_Z3_SOLVER=OFF \
        -DLLVM_ENABLE_FFI=OFF \
        -DLLVM_ENABLE_IDE=OFF
    cmake --build build --config $build_type
    sudo cmake --install build --prefix /usr/local
    cd ../..
}

function vulkan() {
  sudo pacman -S vulkan-tools vulkan-devel spirv-tools
}

function wayland() {
  sudo pacman -S --needed wayland wayland-protocols libxkbcommon-x11 libxkbcommon
}

sudo pacman -Suy

sdl3
glm
miniaudio
zlib
libpng
llvm
vulkan
wayland

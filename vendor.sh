#!/bin/sh
set -e

if [ -z "$1" ]; then
  echo "usage: ./vendor.sh <project-dir>" >&2
  exit 1
fi

# Output Directory
OUT_DIR=$1
VENDOR_OUT_DIR=$OUT_DIR/vendor

# Copy vendor cmake file
SCRIPT_DIR=$(dirname "$0")
cp -r $SCRIPT_DIR/vendor $VENDOR_OUT_DIR

# Bash script for generating a vendor directory with the neccessary properties
git clone https://github.com/libsdl-org/SDL.git $VENDOR_OUT_DIR/SDL
cd $VENDOR_OUT_DIR/SDL; git checkout release-3.4.4; cd -;

# Clone GLFW
git clone https://github.com/glfw/glfw.git $VENDOR_OUT_DIR/glfw
cd $VENDOR_OUT_DIR/glfw; git checkout 3.4; cd -;

# Clone GLM
git clone https://github.com/g-truc/glm.git $VENDOR_OUT_DIR/glm
cd $VENDOR_OUT_DIR/glm; git checkout 1.0.3; cd -;

# Clone LibPNG
git clone https://github.com/pnggroup/libpng.git $VENDOR_OUT_DIR/libpng
cd $VENDOR_OUT_DIR/libpng; git checkout v1.6.58; cd -;

# Clone miniaudio
git clone https://github.com/mackron/miniaudio.git $VENDOR_OUT_DIR/miniaudio
cd $VENDOR_OUT_DIR/miniaudio; git checkout 0.11.25; cd -;

# Clone zlib (libpng, FreeType, and the Rust crates)
git clone https://github.com/madler/zlib.git $VENDOR_OUT_DIR/zlib
cd $VENDOR_OUT_DIR/zlib; git checkout v1.3.2; cd -;

# Clone FreeType
git clone https://github.com/freetype/freetype.git $VENDOR_OUT_DIR/freetype
cd $VENDOR_OUT_DIR/freetype; git checkout VER-2-14-3; cd -;

# Clone LLVM (libraries; the CMake build does not enable the other monorepo projects)
git clone https://github.com/llvm/llvm-project.git $VENDOR_OUT_DIR/llvm-project
cd $VENDOR_OUT_DIR/llvm-project; git checkout llvmorg-23.1.3; cd -;


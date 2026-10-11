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

# Shallow checkout of one tag. Annotated tags peel to the tagged commit.
clone_tag() {
  tag=$1
  url=$2
  dest=$3
  git -c advice.defaultBranchName=false -c init.defaultBranch=main init "$dest"
  git -C "$dest" remote add origin "$url"
  git -C "$dest" fetch --depth 1 origin tag "$tag"
  git -c advice.detachedHead=false -C "$dest" checkout --detach "$tag"
}

clone_tag release-3.4.4 https://github.com/libsdl-org/SDL.git $VENDOR_OUT_DIR/SDL
clone_tag 3.4 https://github.com/glfw/glfw.git $VENDOR_OUT_DIR/glfw
clone_tag 1.0.3 https://github.com/g-truc/glm.git $VENDOR_OUT_DIR/glm
clone_tag v1.6.58 https://github.com/pnggroup/libpng.git $VENDOR_OUT_DIR/libpng
clone_tag 0.11.25 https://github.com/mackron/miniaudio.git $VENDOR_OUT_DIR/miniaudio
clone_tag v1.3.2 https://github.com/madler/zlib.git $VENDOR_OUT_DIR/zlib
clone_tag VER-2-14-3 https://github.com/freetype/freetype.git $VENDOR_OUT_DIR/freetype
clone_tag llvmorg-23.1.3 https://github.com/llvm/llvm-project.git $VENDOR_OUT_DIR/llvm-project


#!/bin/sh

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

cd SDL
cmake -S . -B build -DSDL_STATIC=ON -DCMAKE_BUILD_TYPE=$build_type
cmake --build build --config $build_type
sudo cmake --install build --prefix /usr/local
cd ..

cd glm
cmake -DGLM_BUILD_TESTS=OFF -DBUILD_SHARED_LIBS=OFF -B build .
cmake --build build -- all
sudo cmake --build build -- install
cd ..

cd miniaudio
cmake -B build . -DCMAKE_BUILD_TYPE=$build_type
cmake --build build --config $build_type
sudo cmake --install build
cd ..

cd libpng
cmake -S . -B build -DPNG_SHARED=OFF -DPNG_STATIC=ON -DPNG_BUILD_ZLIB=OFF -DCMAKE_BUILD_TYPE=$build_type
cmake --build build --config $build_type
sudo cmake --install build --prefix /usr/local
cd ..

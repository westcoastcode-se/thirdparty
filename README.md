# thirdparty

## Install pre-requisites

C++ builds need CMake, a C++ compiler, and the system packages used by SDL and GLFW. Rust builds need those plus Cargo, Ninja, pkg-config, and libclang (bindgen reads the installed headers).

```bash
sudo pacman -Syu
sudo pacman -S cmake ninja gcc pkgconf clang rust wayland wayland-protocols libxkbcommon
```

## Checkout this git repository

```bash
git clone --recursive ssh://git@192.168.50.173:3022/westcoastcode-se/thirdparty.git
```

## Add Libraries as Vendor (recommended)

```bash
./vendor.sh ~/Repos/mygame
```

`vendor.sh` copies `vendor/` and checks out the pinned sources into `<project>/vendor/`.

## Build and Install Libraries on System (not recommended)

```bash
./install.sh
```

## Use the libraries from C++

```cmake
add_subdirectory(vendor)
target_link_libraries(your_target PRIVATE SDL3::SDL3-static PNG::PNG glad)
target_include_directories(your_target PRIVATE ${GLAD_INCLUDE_DIR} ${GLM_INCLUDE_DIR} ${LIBPNG_INCLUDE_DIR})
```

## Use the libraries from Rust

The same `vendor/` directory is a Cargo workspace. Depend on one library, or on the `thirdparty` crate for all of them. The build scripts compile the vendored sources, so the Rust program does not use a system copy of SDL, GLFW, glad, GLM, libpng, miniaudio, FreeType, or zlib.

```toml
[dependencies]
thirdparty = { path = "vendor/crates/thirdparty" }

# Or one library:
# sdl3-sys = { path = "vendor/crates/sdl3-sys" }
# glfw-sys = { path = "vendor/crates/glfw-sys" }
# glad-sys = { path = "vendor/crates/glad-sys" }
# glm = { path = "vendor/crates/glm" }
# libpng-sys = { path = "vendor/crates/libpng-sys" }
# miniaudio-sys = { path = "vendor/crates/miniaudio-sys" }
# freetype-sys = { path = "vendor/crates/freetype-sys" }
# zlib-sys = { path = "vendor/crates/zlib-sys" }
```

```rust
let version = unsafe { thirdparty::sdl3_sys::SDL_GetVersion() };
let forward = thirdparty::glm::Vec3::new(0.0, 0.0, -1.0).normalize();
```

`thirdparty` features select libraries: `sdl3`, `glfw`, `glad`, `glm`, `libpng`, `miniaudio`, `freetype`, and `zlib`. The default turns them all on.

GLM is header-only C++. The `glm` crate calls those headers for `Vec2`, `Vec3`, `Vec4`, `Mat4`, and `Quat`. `glm::include_dir()` and the `DEP_GLM_INCLUDE` variable are the header root for other C++ compiled in the same Cargo build.

Wayland support in the Rust SDL and GLFW builds follows `pkg-config`. Without `wayland-protocols`, those two libraries are built with X11 only. Vulkan support in SDL is enabled when the Vulkan headers are installed.

Check the workspace from the project that `vendor.sh` filled:

```bash
cargo test --manifest-path vendor/Cargo.toml
```

Inside this repository the library checkouts are git submodules, and GLFW is not one of them. Point `THIRDPARTY_DIR` at a directory produced by `vendor.sh` before running that command here.

## Libraries

| Library | Version | C++ | Rust crate |
| --- | --- | --- | --- |
| SDL | 3.4.4 | `add_subdirectory(SDL)` | `sdl3-sys` |
| GLFW | 3.4 | `add_subdirectory(glfw)` | `glfw-sys` |
| glad | OpenGL 4.2 | `glad` | `glad-sys` |
| GLM | 1.0.3 | header root `GLM_INCLUDE_DIR` | `glm` |
| libpng | 1.6.58 | `PNG::PNG` | `libpng-sys` |
| miniaudio | 0.11.25 | `add_subdirectory(miniaudio)` | `miniaudio-sys` |
| FreeType | 2.14.3 | sources under `vendor/freetype` | `freetype-sys` |
| zlib | 1.3.2 | sources under `vendor/zlib` | `zlib-sys` |

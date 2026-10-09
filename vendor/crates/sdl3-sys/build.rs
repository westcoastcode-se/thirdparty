use std::path::Path;

fn main() {
    let mut defines = vec![
        ("SDL_SHARED", "OFF"),
        ("SDL_STATIC", "ON"),
        ("SDL_TEST_LIBRARY", "OFF"),
        ("SDL_TESTS", "OFF"),
        ("SDL_EXAMPLES", "OFF"),
    ];
    if !thirdparty_build::pkg_config_exists("wayland-protocols") {
        println!("cargo:warning=wayland-protocols was not found; SDL is built without Wayland");
        defines.push(("SDL_WAYLAND", "OFF"));
    }
    if !Path::new("/usr/include/vulkan/vulkan.h").exists()
        && !thirdparty_build::pkg_config_exists("vulkan")
    {
        println!("cargo:warning=Vulkan headers were not found; SDL is built without Vulkan");
        defines.push(("SDL_VULKAN", "OFF"));
    }
    thirdparty_build::cmake_ffi(&thirdparty_build::CmakeFfi {
        dir_name: "SDL",
        marker: "include/SDL3/SDL.h",
        pc_package: "sdl3",
        wrapper_h: "#include <SDL3/SDL.h>\n",
        header_patterns: &[r".*SDL3[\\/].*\.h"],
        expect_symbol: "SDL_GetVersion",
        defines: &defines,
        prefix_paths: &[],
    });
}

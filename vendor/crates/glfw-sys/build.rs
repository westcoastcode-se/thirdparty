fn main() {
    let mut defines = vec![
        ("GLFW_BUILD_EXAMPLES", "OFF"),
        ("GLFW_BUILD_TESTS", "OFF"),
        ("GLFW_BUILD_DOCS", "OFF"),
        ("GLFW_INSTALL", "ON"),
        ("GLFW_LIBRARY_TYPE", "STATIC"),
        ("GLFW_BUILD_X11", "ON"),
    ];
    if !thirdparty_build::pkg_config_exists("wayland-protocols") {
        println!("cargo:warning=wayland-protocols was not found; GLFW is built without Wayland");
        defines.push(("GLFW_BUILD_WAYLAND", "OFF"));
    }
    thirdparty_build::cmake_ffi(&thirdparty_build::CmakeFfi {
        dir_name: "glfw",
        marker: "include/GLFW/glfw3.h",
        pc_package: "glfw3",
        wrapper_h: "#include <GLFW/glfw3.h>\n",
        header_patterns: &[r".*GLFW[\\/].*\.h"],
        expect_symbol: "glfwGetVersion",
        defines: &defines,
        prefix_paths: &[],
    });
}

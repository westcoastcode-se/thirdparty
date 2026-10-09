use std::env;
use std::path::PathBuf;

fn main() {
    let zlib_root =
        PathBuf::from(env::var("DEP_Z_ROOT").expect("zlib-sys did not export its install prefix"));
    let zlib_root_text = zlib_root.display().to_string();
    let defines = [
        ("PNG_SHARED", "OFF"),
        ("PNG_STATIC", "ON"),
        ("PNG_TOOLS", "OFF"),
        ("PNG_TESTS", "OFF"),
        ("PNG_FRAMEWORK", "OFF"),
        ("PNG_DEBUG_POSTFIX", ""),
        ("ZLIB_USE_STATIC_LIBS", "ON"),
        ("ZLIB_ROOT", zlib_root_text.as_str()),
    ];
    thirdparty_build::cmake_ffi(&thirdparty_build::CmakeFfi {
        dir_name: "libpng",
        marker: "png.h",
        pc_package: "libpng",
        wrapper_h: "#include <png.h>\n",
        header_patterns: &[
            r".*[\\/]png\.h",
            r".*[\\/]pngconf\.h",
            r".*[\\/]pnglibconf\.h",
        ],
        expect_symbol: "png_access_version_number",
        defines: &defines,
        prefix_paths: &[zlib_root],
    });
}

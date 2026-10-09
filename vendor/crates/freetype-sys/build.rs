use std::env;
use std::path::PathBuf;

fn main() {
    let zlib_root =
        PathBuf::from(env::var("DEP_Z_ROOT").expect("zlib-sys did not export its install prefix"));
    let zlib_root_text = zlib_root.display().to_string();
    // External zlib is the same static library as zlib-sys. The bundled copy
    // would export the same symbols and fail the link.
    let defines = [
        ("FT_DISABLE_ZLIB", "OFF"),
        ("FT_REQUIRE_ZLIB", "ON"),
        ("FT_DISABLE_BZIP2", "ON"),
        ("FT_DISABLE_PNG", "ON"),
        ("FT_DISABLE_HARFBUZZ", "ON"),
        ("FT_DISABLE_BROTLI", "ON"),
        ("DISABLE_FORCE_DEBUG_POSTFIX", "ON"),
        ("ZLIB_USE_STATIC_LIBS", "ON"),
        ("ZLIB_ROOT", zlib_root_text.as_str()),
    ];
    thirdparty_build::cmake_ffi(&thirdparty_build::CmakeFfi {
        dir_name: "freetype",
        marker: "include/freetype/freetype.h",
        pc_package: "freetype2",
        wrapper_h: "#include <ft2build.h>\n#include FT_FREETYPE_H\n",
        header_patterns: &[r".*freetype2[\\/].*\.h"],
        expect_symbol: "FT_Init_FreeType",
        defines: &defines,
        prefix_paths: &[zlib_root],
    });
}

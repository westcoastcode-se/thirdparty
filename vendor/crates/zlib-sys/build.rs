fn main() {
    thirdparty_build::cmake_ffi(&thirdparty_build::CmakeFfi {
        dir_name: "zlib",
        marker: "zlib.h",
        pc_package: "zlib",
        wrapper_h: "#include <zlib.h>\n",
        header_patterns: &[r".*[\\/]zlib\.h", r".*[\\/]zconf\.h"],
        expect_symbol: "zlibVersion",
        defines: &[
            ("ZLIB_BUILD_SHARED", "OFF"),
            ("ZLIB_BUILD_STATIC", "ON"),
            ("ZLIB_BUILD_TESTING", "OFF"),
            ("ZLIB_INSTALL", "ON"),
        ],
        prefix_paths: &[],
    });
}

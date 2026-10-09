fn main() {
    thirdparty_build::cmake_ffi(&thirdparty_build::CmakeFfi {
        dir_name: "miniaudio",
        marker: "miniaudio.h",
        pc_package: "miniaudio",
        wrapper_h: "#include <miniaudio/miniaudio.h>\n",
        header_patterns: &[r".*[\\/]miniaudio\.h"],
        expect_symbol: "ma_version",
        defines: &[
            ("MINIAUDIO_BUILD_EXAMPLES", "OFF"),
            ("MINIAUDIO_BUILD_TESTS", "OFF"),
            ("MINIAUDIO_BUILD_TOOLS", "OFF"),
            ("MINIAUDIO_NO_EXTRA_NODES", "ON"),
            ("MINIAUDIO_NO_LIBVORBIS", "ON"),
            ("MINIAUDIO_NO_LIBOPUS", "ON"),
        ],
        prefix_paths: &[],
    });
}

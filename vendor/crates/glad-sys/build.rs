fn main() {
    println!("cargo:rerun-if-env-changed=THIRDPARTY_DIR");
    let source = thirdparty_build::source_dir("glad", "include/glad/glad.h");
    let include = source.join("include");
    cc::Build::new()
        .file(source.join("src/glad.c"))
        .include(&include)
        .warnings(false)
        .compile("glad");
    println!("cargo:include={}", include.display());
    println!("cargo:root={}", source.display());

    let include_arg = format!("-I{}", include.display());
    thirdparty_build::generate_bindings(&thirdparty_build::Bindings {
        wrapper_h: "#include <glad/glad.h>\n",
        clang_args: &[include_arg],
        header_patterns: &[r".*glad[\\/]glad\.h", r".*KHR[\\/]khrplatform\.h"],
        expect_symbol: "gladLoadGL",
    });
}

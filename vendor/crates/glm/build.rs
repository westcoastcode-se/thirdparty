fn main() {
    println!("cargo:rerun-if-env-changed=THIRDPARTY_DIR");
    let source = thirdparty_build::source_dir("glm", "glm/glm.hpp");
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .file("src/glm_c.cpp")
        .include(&source)
        .warnings(false)
        .compile("glm_c");
    println!("cargo:include={}", source.display());
    println!("cargo:root={}", source.display());
    println!("cargo:rerun-if-changed=src/glm_c.cpp");
    println!("cargo:rustc-env=GLM_INCLUDE={}", source.display());
}

//! Shared build logic for the vendored libraries.
//!
//! Each sys crate lives in `vendor/crates/<name>`. `vendor.sh` clones the
//! library sources next to `crates/`, so a crate finds `vendor/<library>`.
//! `THIRDPARTY_DIR` overrides that root. A checkout of this repository can
//! also use the git submodules at the repository root.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn source_dir(dir_name: &str, marker: &str) -> PathBuf {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let vendor_dir = manifest
        .parent()
        .and_then(Path::parent)
        .expect("crate must live in vendor/crates/<name>")
        .to_path_buf();
    let repo_root = vendor_dir
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| vendor_dir.clone());

    let mut candidates = vec![vendor_dir.join(dir_name), repo_root.join(dir_name)];
    if let Ok(root) = env::var("THIRDPARTY_DIR") {
        candidates.push(PathBuf::from(root).join(dir_name));
    }

    for candidate in &candidates {
        if candidate.join(marker).is_file() {
            println!(
                "cargo:rerun-if-changed={}",
                candidate.join(marker).display()
            );
            let cmake = candidate.join("CMakeLists.txt");
            if cmake.is_file() {
                println!("cargo:rerun-if-changed={}", cmake.display());
            }
            return candidate.clone();
        }
    }

    panic!(
        "could not find {dir_name} (expected {marker}). Looked in:\n{}",
        candidates
            .iter()
            .map(|path| format!("  {}", path.display()))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

pub fn pkg_config_exists(package: &str) -> bool {
    Command::new("pkg-config")
        .arg("--exists")
        .arg(package)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub struct CmakeFfi<'a> {
    pub dir_name: &'a str,
    pub marker: &'a str,
    pub pc_package: &'a str,
    pub wrapper_h: &'a str,
    pub header_patterns: &'a [&'a str],
    pub expect_symbol: &'a str,
    pub defines: &'a [(&'a str, &'a str)],
    /// Install prefixes of libraries this build must find, such as zlib.
    pub prefix_paths: &'a [PathBuf],
}

pub fn cmake_ffi(spec: &CmakeFfi<'_>) {
    println!("cargo:rerun-if-env-changed=THIRDPARTY_DIR");
    let source = source_dir(spec.dir_name, spec.marker);
    let prefix = cmake_install(&source, spec.defines, spec.prefix_paths);
    println!("cargo:root={}", prefix.display());

    let pc_dirs = pkgconfig_dirs(&prefix, spec.prefix_paths);
    link_static_package(&prefix, &pc_dirs, spec.pc_package);
    let cflags = pkg_config_output(&pc_dirs, &["--cflags", spec.pc_package]);
    let mut clang_args = Vec::new();
    for token in cflags.split_whitespace() {
        if let Some(dir) = token.strip_prefix("-I") {
            println!("cargo:include={dir}");
        }
        clang_args.push(token.to_string());
    }
    generate_bindings(&Bindings {
        wrapper_h: spec.wrapper_h,
        clang_args: &clang_args,
        header_patterns: spec.header_patterns,
        expect_symbol: spec.expect_symbol,
    });
}

pub struct Bindings<'a> {
    pub wrapper_h: &'a str,
    pub clang_args: &'a [String],
    pub header_patterns: &'a [&'a str],
    pub expect_symbol: &'a str,
}

pub fn generate_bindings(spec: &Bindings<'_>) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let wrapper = out_dir.join("wrapper.h");
    fs::write(&wrapper, spec.wrapper_h).expect("write bindgen wrapper");

    let mut builder = bindgen::Builder::default()
        .header(wrapper.to_string_lossy())
        .generate_comments(false)
        .layout_tests(false)
        .prepend_enum_name(false)
        .formatter(bindgen::Formatter::None)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    for arg in spec.clang_args {
        builder = builder.clang_arg(arg);
    }
    for pattern in spec.header_patterns {
        builder = builder.allowlist_file(*pattern);
    }

    let bindings = builder.generate().unwrap_or_else(|error| {
        panic!(
            "bindgen failed for {} ({error}). clang args: {}",
            spec.expect_symbol,
            spec.clang_args.join(" ")
        )
    });
    let bindings_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("write bindings.rs");
    let generated = fs::read_to_string(&bindings_path).expect("read bindings.rs");
    if !generated.contains(spec.expect_symbol) {
        panic!(
            "generated bindings do not contain `{}`. The header allow-list did not match the installed headers.",
            spec.expect_symbol
        );
    }
}

fn cmake_install(source: &Path, defines: &[(&str, &str)], prefix_paths: &[PathBuf]) -> PathBuf {
    let mut config = cmake::Config::new(source);
    config
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("CMAKE_DEBUG_POSTFIX", "")
        .define("BUILD_SHARED_LIBS", "OFF");
    for (key, value) in defines {
        config.define(*key, *value);
    }
    if !prefix_paths.is_empty() {
        let joined = prefix_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(";");
        config.define("CMAKE_PREFIX_PATH", &joined);
        let pc = prefix_paths
            .iter()
            .map(|path| path.join("lib/pkgconfig").display().to_string())
            .collect::<Vec<_>>()
            .join(":");
        config.env("PKG_CONFIG_PATH", &pc);
    }
    if Command::new("ninja")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        config.generator("Ninja");
    }
    config.build()
}

fn pkgconfig_dirs(prefix: &Path, extra_prefixes: &[PathBuf]) -> Vec<PathBuf> {
    let mut dirs = vec![prefix.join("lib/pkgconfig")];
    for extra in extra_prefixes {
        dirs.push(extra.join("lib/pkgconfig"));
    }
    dirs
}

fn pkg_config_output(pc_dirs: &[PathBuf], args: &[&str]) -> String {
    let mut paths = pc_dirs
        .iter()
        .map(|dir| dir.display().to_string())
        .collect::<Vec<_>>();
    if let Ok(existing) = env::var("PKG_CONFIG_PATH") {
        if !existing.is_empty() {
            paths.push(existing);
        }
    }
    let output = Command::new("pkg-config")
        .env("PKG_CONFIG_PATH", paths.join(":"))
        .env("PKG_CONFIG_ALLOW_SYSTEM_LIBS", "1")
        .env("PKG_CONFIG_ALLOW_SYSTEM_CFLAGS", "1")
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("failed to run pkg-config: {error}"));
    if !output.status.success() {
        panic!(
            "pkg-config {} failed:\n{}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    String::from_utf8(output.stdout).expect("pkg-config output is utf-8")
}

fn link_static_package(prefix: &Path, pc_dirs: &[PathBuf], package: &str) {
    let flags = pkg_config_output(pc_dirs, &["--static", "--libs", package]);
    let target_dir = cargo_target_dir();
    let mut search_dirs = Vec::new();
    let mut libs = Vec::new();
    let mut tokens = flags.split_whitespace().peekable();
    while let Some(token) = tokens.next() {
        if let Some(dir) = token.strip_prefix("-L") {
            search_dirs.push(PathBuf::from(dir));
        } else if let Some(name) = token.strip_prefix("-l") {
            if name.contains('/') || name.contains('\\') {
                link_absolute_library(name);
            } else {
                libs.push(name.to_string());
            }
        } else if token.starts_with('/')
            && (token.contains(".so") || token.ends_with(".a") || token.ends_with(".lib"))
        {
            link_absolute_library(token);
        } else if token == "-pthread" {
            println!("cargo:rustc-link-arg=-pthread");
        } else if token == "-framework" {
            if let Some(framework) = tokens.next() {
                println!("cargo:rustc-link-arg=-framework");
                println!("cargo:rustc-link-arg={framework}");
            }
        } else if token.starts_with("-Wl") {
            println!("cargo:rustc-link-arg={token}");
        }
    }

    // The install prefix is authoritative even when pkg-config expands a
    // relative libdir that does not point at the archive.
    let prefix_lib = prefix.join("lib");
    if !search_dirs.iter().any(|dir| dir == &prefix_lib) {
        search_dirs.insert(0, prefix_lib);
    }
    for dir in &search_dirs {
        println!("cargo:rustc-link-search=native={}", dir.display());
    }
    if libs.is_empty() {
        panic!("pkg-config --libs {package} did not list any libraries");
    }
    for name in libs {
        if static_archive(&name, &search_dirs, &target_dir) {
            println!("cargo:rustc-link-lib=static={name}");
        } else {
            println!("cargo:rustc-link-lib=dylib={name}");
        }
    }
}

fn link_absolute_library(path: &str) {
    let file = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    if let Some(rest) = file.strip_prefix("lib") {
        if let Some(libname) = rest.split(['.', '-']).next() {
            if !libname.is_empty() && (path.starts_with("/usr") || path.starts_with("/lib")) {
                println!("cargo:rustc-link-lib=dylib={libname}");
                return;
            }
        }
    }
    println!("cargo:rustc-link-arg={path}");
}

fn static_archive(name: &str, dirs: &[PathBuf], target_dir: &Path) -> bool {
    for dir in dirs {
        if !dir.starts_with(target_dir) {
            continue;
        }
        if dir.join(format!("lib{name}.a")).is_file() || dir.join(format!("{name}.lib")).is_file() {
            return true;
        }
    }
    false
}

fn cargo_target_dir() -> PathBuf {
    if let Ok(dir) = env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(dir);
    }
    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    // target/<profile>/build/<crate>/out
    out.ancestors()
        .nth(4)
        .expect("OUT_DIR is not inside a Cargo target directory")
        .to_path_buf()
}

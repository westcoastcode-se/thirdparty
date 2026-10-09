use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// Keep these in sync with vendor/llvm.cmake and install.sh. LLVM publishes no
// pkg-config file, and a Debug build of the libraries is too large for the
// Rust workspace, so this crate configures CMake itself.
const DEFINES: &[(&str, &str)] = &[
    ("CMAKE_INSTALL_LIBDIR", "lib"),
    ("BUILD_SHARED_LIBS", "OFF"),
    ("LLVM_ENABLE_PROJECTS", ""),
    ("LLVM_ENABLE_RUNTIMES", ""),
    ("LLVM_TARGETS_TO_BUILD", "Native"),
    ("LLVM_APPEND_VC_REV", "OFF"),
    ("LLVM_BUILD_TOOLS", "OFF"),
    ("LLVM_INCLUDE_TOOLS", "OFF"),
    ("LLVM_BUILD_UTILS", "OFF"),
    ("LLVM_INCLUDE_UTILS", "OFF"),
    ("LLVM_BUILD_RUNTIME", "OFF"),
    ("LLVM_INCLUDE_RUNTIMES", "OFF"),
    ("LLVM_BUILD_EXAMPLES", "OFF"),
    ("LLVM_INCLUDE_EXAMPLES", "OFF"),
    ("LLVM_BUILD_TESTS", "OFF"),
    ("LLVM_INCLUDE_TESTS", "OFF"),
    ("LLVM_BUILD_BENCHMARKS", "OFF"),
    ("LLVM_INCLUDE_BENCHMARKS", "OFF"),
    ("LLVM_BUILD_DOCS", "OFF"),
    ("LLVM_INCLUDE_DOCS", "OFF"),
    ("LLVM_ENABLE_BINDINGS", "OFF"),
    ("LLVM_ENABLE_OCAMLDOC", "OFF"),
    ("LLVM_ENABLE_TELEMETRY", "OFF"),
    ("LLVM_ENABLE_PLUGINS", "OFF"),
    ("LLVM_BUILD_LLVM_DYLIB", "OFF"),
    ("LLVM_BUILD_LLVM_C_DYLIB", "OFF"),
    ("LLVM_ENABLE_ZLIB", "OFF"),
    ("LLVM_ENABLE_ZSTD", "OFF"),
    ("LLVM_ENABLE_LIBXML2", "OFF"),
    ("LLVM_ENABLE_LIBEDIT", "OFF"),
    ("LLVM_ENABLE_LIBPFM", "OFF"),
    ("LLVM_ENABLE_CURL", "OFF"),
    ("LLVM_ENABLE_HTTPLIB", "OFF"),
    ("LLVM_ENABLE_Z3_SOLVER", "OFF"),
    ("LLVM_ENABLE_FFI", "OFF"),
    ("LLVM_ENABLE_IDE", "OFF"),
    // Cap compile parallelism from available RAM when the caller does not.
    ("LLVM_RAM_PER_COMPILE_JOB", "2000"),
];

const WRAPPER: &str = r#"
#include <llvm-c/Analysis.h>
#include <llvm-c/BitReader.h>
#include <llvm-c/BitWriter.h>
#include <llvm-c/Comdat.h>
#include <llvm-c/Core.h>
#include <llvm-c/DebugInfo.h>
#include <llvm-c/Disassembler.h>
#include <llvm-c/Error.h>
#include <llvm-c/ErrorHandling.h>
#include <llvm-c/ExecutionEngine.h>
#include <llvm-c/IRReader.h>
#include <llvm-c/LLJIT.h>
#include <llvm-c/LLJITUtils.h>
#include <llvm-c/Linker.h>
#include <llvm-c/Object.h>
#include <llvm-c/Orc.h>
#include <llvm-c/OrcEE.h>
#include <llvm-c/Remarks.h>
#include <llvm-c/Support.h>
#include <llvm-c/Target.h>
#include <llvm-c/TargetMachine.h>
#include <llvm-c/Transforms/PassBuilder.h>
"#;

fn main() {
    println!("cargo:rerun-if-env-changed=THIRDPARTY_DIR");
    let source = thirdparty_build::source_dir("llvm-project", "llvm/include/llvm-c/Core.h");
    let llvm_src = source.join("llvm");
    println!(
        "cargo:rerun-if-changed={}",
        llvm_src.join("CMakeLists.txt").display()
    );

    let prefix = install_llvm(&llvm_src);
    println!("cargo:root={}", prefix.display());
    let include = prefix.join("include");
    println!("cargo:include={}", include.display());
    link_llvm(&prefix);

    let clang_args = vec![format!("-I{}", include.display())];
    thirdparty_build::generate_bindings(&thirdparty_build::Bindings {
        wrapper_h: WRAPPER,
        clang_args: &clang_args,
        header_patterns: &[r".*llvm-c[\\/].*\.h"],
        expect_symbol: "LLVMGetVersion",
    });
}

fn install_llvm(source: &Path) -> PathBuf {
    let mut config = cmake::Config::new(source);
    config.define("CMAKE_DEBUG_POSTFIX", "").profile("Release");
    for (key, value) in DEFINES {
        config.define(*key, *value);
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

fn link_llvm(prefix: &Path) {
    let lib_dir = ["lib", "lib64"]
        .iter()
        .map(|name| prefix.join(name))
        .find(|dir| dir.is_dir())
        .unwrap_or_else(|| panic!("LLVM install at {} has no lib directory", prefix.display()));

    let mut archives = Vec::new();
    for entry in fs::read_dir(&lib_dir).expect("read LLVM lib directory") {
        let entry = entry.expect("read LLVM lib entry");
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with("libLLVM") && name.ends_with(".a") && name != "libLLVMAll.a" {
            archives.push(entry.path());
        }
    }
    archives.sort();
    if archives.is_empty() {
        panic!(
            "LLVM install at {} contains no static libraries",
            lib_dir.display()
        );
    }

    // The component archives refer to each other. Rust rejects a linker
    // script disguised as an archive, so pack every object into one archive
    // that dependents can link with a single static library.
    merge_archives(&lib_dir, &archives);

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=LLVMAll");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=pthread");
}

fn merge_archives(lib_dir: &Path, archives: &[PathBuf]) {
    let scratch = lib_dir.join("llvm-all-objects");
    let _ = fs::remove_dir_all(&scratch);
    fs::create_dir(&scratch).expect("create LLVM object scratch directory");

    let mut members = Vec::new();
    for (index, archive) in archives.iter().enumerate() {
        let extract = scratch.join(format!("a{index}"));
        fs::create_dir(&extract).expect("create LLVM extract directory");
        let status = Command::new("ar")
            .arg("x")
            .arg(archive)
            .current_dir(&extract)
            .status()
            .unwrap_or_else(|error| panic!("failed to run ar: {error}"));
        if !status.success() {
            panic!("ar x {} failed", archive.display());
        }
        let mut extracted = fs::read_dir(&extract)
            .expect("read extracted LLVM objects")
            .map(|entry| entry.expect("read extracted LLVM object").path())
            .collect::<Vec<_>>();
        extracted.sort();
        for path in extracted {
            if !path.is_file() {
                continue;
            }
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("object");
            let unique = scratch.join(format!("{index}-{file_name}"));
            fs::rename(&path, &unique).expect("rename LLVM object");
            members.push(unique);
        }
        let _ = fs::remove_dir(&extract);
    }
    if members.is_empty() {
        panic!("LLVM static libraries did not contain any objects");
    }

    let combined = lib_dir.join("libLLVMAll.a");
    let _ = fs::remove_file(&combined);
    let script_path = scratch.join("merge.mri");
    let mut script = format!("CREATE {}\n", combined.display());
    for member in &members {
        script.push_str(&format!("ADDMOD {}\n", member.display()));
    }
    script.push_str("SAVE\nEND\n");
    fs::write(&script_path, &script).expect("write ar MRI script");
    let status = Command::new("ar")
        .arg("-M")
        .stdin(fs::File::open(&script_path).expect("open ar MRI script"))
        .status()
        .unwrap_or_else(|error| panic!("failed to run ar: {error}"));
    if !status.success() {
        panic!("ar could not merge the LLVM static libraries");
    }
    let _ = fs::remove_dir_all(&scratch);
}

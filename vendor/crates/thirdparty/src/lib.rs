//! Re-exports of the vendored libraries.
//!
//! Depend on this crate to link the whole set, or depend on one sys crate
//! under `vendor/crates/` when a project only needs that library.

#[cfg(feature = "freetype")]
pub use freetype_sys;
#[cfg(feature = "glad")]
pub use glad_sys;
#[cfg(feature = "glfw")]
pub use glfw_sys;
#[cfg(feature = "glm")]
pub use glm;
#[cfg(feature = "libpng")]
pub use libpng_sys;
#[cfg(feature = "llvm")]
pub use llvm_sys;
#[cfg(feature = "miniaudio")]
pub use miniaudio_sys;
#[cfg(feature = "sdl3")]
pub use sdl3_sys;
#[cfg(feature = "zlib")]
pub use zlib_sys;

#[cfg(all(
    feature = "sdl3",
    feature = "glfw",
    feature = "glad",
    feature = "glm",
    feature = "libpng",
    feature = "miniaudio",
    feature = "freetype",
    feature = "zlib",
    feature = "llvm"
))]
#[cfg(test)]
mod tests {
    use crate::{
        freetype_sys, glad_sys, glfw_sys, glm, libpng_sys, llvm_sys, miniaudio_sys, sdl3_sys,
        zlib_sys,
    };

    #[test]
    fn one_binary_links_every_vendored_library() {
        unsafe {
            let sdl = sdl3_sys::SDL_GetVersion();
            assert_eq!(sdl / 1_000_000, 3);

            let mut major = 0;
            let mut minor = 0;
            let mut revision = 0;
            glfw_sys::glfwGetVersion(&mut major, &mut minor, &mut revision);
            assert_eq!(major, 3);

            assert_eq!(glad_sys::GL_TRIANGLES, 0x0004);
            let mut ma_major = 0u32;
            let mut ma_minor = 0u32;
            let mut ma_revision = 0u32;
            miniaudio_sys::ma_version(&mut ma_major, &mut ma_minor, &mut ma_revision);
            assert_eq!((ma_major, ma_minor, ma_revision), (0, 11, 25));

            assert_eq!(libpng_sys::png_access_version_number(), 10658);
            let version = std::ffi::CStr::from_ptr(zlib_sys::zlibVersion());
            assert_eq!(version.to_str().unwrap(), "1.3.2");

            let mut library = std::ptr::null_mut();
            assert_eq!(freetype_sys::FT_Init_FreeType(&mut library), 0);
            freetype_sys::FT_Done_FreeType(library);

            let mut llvm_major = 0u32;
            let mut llvm_minor = 0u32;
            let mut llvm_patch = 0u32;
            llvm_sys::LLVMGetVersion(&mut llvm_major, &mut llvm_minor, &mut llvm_patch);
            assert_eq!((llvm_major, llvm_minor, llvm_patch), (23, 1, 3));
        }

        let normal = glm::Vec3::new(0.0, 3.0, 0.0).normalize();
        assert!((normal.y - 1.0).abs() < 1e-5);
    }
}

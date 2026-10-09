#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused,
    unnecessary_transmutes,
    clippy::all
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_vendored_freetype() {
        let mut library: FT_Library = std::ptr::null_mut();
        let status = unsafe { FT_Init_FreeType(&mut library) };
        assert_eq!(status, 0);
        assert!(!library.is_null());

        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        unsafe { FT_Library_Version(library, &mut major, &mut minor, &mut patch) };
        assert_eq!((major, minor, patch), (2, 14, 3));
        unsafe { FT_Done_FreeType(library) };
    }
}

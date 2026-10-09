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
    fn reports_vendored_version_and_creates_a_reader() {
        let version = unsafe { png_access_version_number() };
        assert_eq!(version, 10658);

        let user_version = c"1.6.58";
        let png = unsafe {
            png_create_read_struct(user_version.as_ptr(), std::ptr::null_mut(), None, None)
        };
        assert!(!png.is_null());
        let mut png = png;
        let mut info = std::ptr::null_mut();
        let mut end = std::ptr::null_mut();
        unsafe { png_destroy_read_struct(&mut png, &mut info, &mut end) };
    }
}

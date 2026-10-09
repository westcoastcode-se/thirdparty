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
    fn reports_vendored_version() {
        let mut major = 0u32;
        let mut minor = 0u32;
        let mut revision = 0u32;
        unsafe { ma_version(&mut major, &mut minor, &mut revision) };
        assert_eq!((major, minor, revision), (0, 11, 25));
    }
}

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
    fn reports_vendored_version_without_initializing() {
        let version = unsafe { SDL_GetVersion() };
        let major = version / 1_000_000;
        let minor = (version / 1_000) % 1_000;
        let patch = version % 1_000;
        assert_eq!((major, minor, patch), (3, 4, 4));
    }
}

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
    fn exposes_the_loader_and_core_tokens() {
        assert_eq!(GL_TRIANGLES, 0x0004);
        let _loader: unsafe extern "C" fn() -> i32 = gladLoadGL;
    }
}

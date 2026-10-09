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
    fn reports_vendored_llvm_version() {
        let mut major = 0u32;
        let mut minor = 0u32;
        let mut patch = 0u32;
        unsafe { LLVMGetVersion(&mut major, &mut minor, &mut patch) };
        assert_eq!((major, minor, patch), (23, 1, 3));
    }
}

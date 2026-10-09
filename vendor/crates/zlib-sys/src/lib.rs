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
    use std::ffi::CStr;

    #[test]
    fn reports_vendored_version_and_roundtrips() {
        let version = unsafe { CStr::from_ptr(zlibVersion()) };
        assert_eq!(version.to_str().unwrap(), "1.3.2");

        let source = b"hello hello hello hello hello hello";
        let mut compressed = vec![0u8; 128];
        let mut compressed_len = compressed.len() as uLong;
        let status = unsafe {
            compress(
                compressed.as_mut_ptr(),
                &mut compressed_len,
                source.as_ptr(),
                source.len() as uLong,
            )
        };
        assert_eq!(status, Z_OK as i32);

        let mut restored = vec![0u8; source.len()];
        let mut restored_len = restored.len() as uLong;
        let status = unsafe {
            uncompress(
                restored.as_mut_ptr(),
                &mut restored_len,
                compressed.as_ptr(),
                compressed_len,
            )
        };
        assert_eq!(status, Z_OK as i32);
        assert_eq!(&restored[..restored_len as usize], source);
    }
}

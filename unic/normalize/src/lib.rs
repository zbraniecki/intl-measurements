pub mod ffi;

use unicode_normalization::UnicodeNormalization;

#[inline(never)]
pub fn icu(normalizer: *mut libc::c_void, src: &[u16], dst: &mut [u16]) {
    let mut err = 0;

    let _ = unsafe {
        ffi::unorm2_normalize_67(
            normalizer,
            src.as_ptr(),
            src.len() as i32,
            dst.as_mut_ptr(),
            dst.len() as i32,
            &mut err,
        );
    };
}

pub fn write_to_utf16_slice<'a, I: IntoIterator<Item = char>>(iter: I, slice: &mut [u16]) {
    let mut tail = slice;
    for c in iter {
        let written = c.encode_utf16(tail).len();
        tail = &mut tail[written..];
    }
}

#[inline(never)]
pub fn nfc_rust_utf8(src: &str, dst: &mut String) {
    dst.extend(src.nfc());
}

#[inline(never)]
pub fn nfc_rust_utf16(src: &[u16], dst: &mut [u16]) {
    write_to_utf16_slice(
        std::char::decode_utf16(src.into_iter().cloned())
            .map(|r| r.unwrap_or('\u{FFFD}'))
            .nfc(),
        dst,
    );
}

#[inline(never)]
pub fn nfd_rust_utf8(src: &str, dst: &mut String) {
    dst.extend(src.nfd());
}

#[inline(never)]
pub fn nfd_rust_utf16(src: &[u16], dst: &mut [u16]) {
    write_to_utf16_slice(
        std::char::decode_utf16(src.into_iter().cloned())
            .map(|r| r.unwrap_or('\u{FFFD}'))
            .nfd(),
        dst,
    );
}

#[inline(never)]
pub fn nfc_rust_utf16_count_only(src: &[u16]) -> usize {
    std::char::decode_utf16(src.into_iter().cloned())
            .map(|r| r.unwrap_or('\u{FFFD}'))
            .nfc().count()
}

#[inline(never)]
pub fn nfd_rust_utf16_count_only(src: &[u16]) -> usize {
    std::char::decode_utf16(src.into_iter().cloned())
            .map(|r| r.unwrap_or('\u{FFFD}'))
            .nfd().count()
}


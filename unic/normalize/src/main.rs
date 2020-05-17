use std::fs;
use std::time::Instant;

use detone::IterDecomposeVietnamese;

use unicode_normalization::UnicodeNormalization;

#[link(name = "icuuc")]
extern "C" {
    fn unorm2_getNFCInstance_67(pErrorCode: *mut libc::c_int) -> *mut libc::c_void;
    fn unorm2_getNFDInstance_67(pErrorCode: *mut libc::c_int) -> *mut libc::c_void;
    fn unorm2_close_67(norm2: *mut libc::c_void);
    fn unorm2_normalize_67(
        norm2: *mut libc::c_void,
        src: *const u16,
        length: i32,
        dest: *mut u16,
        capacity: i32,
        pErrorCode: *mut libc::c_int,
    ) -> i32;
}

fn write_to_utf16_slice<'a, I: IntoIterator<Item = char>>(iter: I, slice: &mut [u16]) {
    let mut tail = slice;
    for c in iter {
        let written = c.encode_utf16(tail).len();
        tail = &mut tail[written..];
    }
}

#[inline(never)]
fn normalize_nfc_rust(src: &[u16], dst: &mut [u16]) {
    write_to_utf16_slice(
        std::char::decode_utf16(src.into_iter().cloned())
            .map(|r| r.unwrap_or('\u{FFFD}'))
            .nfc(),
        dst,
    );
}

#[inline(never)]
fn normalize_nfd_rust(src: &[u16], dst: &mut [u16]) {
    write_to_utf16_slice(
        std::char::decode_utf16(src.into_iter().cloned())
            .map(|r| r.unwrap_or('\u{FFFD}'))
            .nfd(),
        dst,
    );
}

#[inline(never)]
fn normalize_nfc_rust_count_only(src: &[u16]) -> usize {
    std::char::decode_utf16(src.into_iter().cloned())
            .map(|r| r.unwrap_or('\u{FFFD}'))
            .nfc().count()
}

#[inline(never)]
fn normalize_nfd_rust_count_only(src: &[u16]) -> usize {
    std::char::decode_utf16(src.into_iter().cloned())
            .map(|r| r.unwrap_or('\u{FFFD}'))
            .nfd().count()
}

fn normalize_icu(normalizer: *mut libc::c_void, src: &[u16], dst: &mut [u16]) {
    let mut err = 0;

    let _ = unsafe {
        unorm2_normalize_67(
            normalizer,
            src.as_ptr(),
            src.len() as i32,
            dst.as_mut_ptr(),
            dst.len() as i32,
            &mut err,
        );
    };
}

fn measure_nfc(loc: &str, sample: &str, form: &str) {
    let mut err = 0;
    let norm = unsafe { unorm2_getNFCInstance_67(&mut err) };
    let src: Vec<u16> = sample.encode_utf16().collect();
    let mut dest: Vec<u16> = Vec::new();
    dest.resize(src.len() * 10, 0); // Assume the operation won't stretch the string by a factor larger than 10

    let now = Instant::now();

    normalize_icu(norm, &src, &mut dest);

    let measured_us = now.elapsed().as_micros();

    let now = Instant::now();

    normalize_nfc_rust(&src, &mut dest);

    let measured_us2 = now.elapsed().as_micros();

    let now = Instant::now();

    let count = normalize_nfc_rust_count_only(&src);

    let measured_us3 = now.elapsed().as_micros();

    println!(
        "Normalization to NFC from {} of {} sample. ICU: {} us, Rust: {} us, Rust count {}: {} us",
        form, loc, measured_us, measured_us2, count, measured_us3
    );

    assert!(count < dest.len());

    // unsafe { unorm2_close_67(norm); }
}

fn measure_nfd(loc: &str, sample: &str, form: &str) {
    let mut err = 0;
    let norm = unsafe { unorm2_getNFDInstance_67(&mut err) };
    let src: Vec<u16> = sample.encode_utf16().collect();
    let mut dest: Vec<u16> = Vec::new();
    dest.resize(src.len() * 10, 0); // Assume the operation won't stretch the string by a factor larger than 10

    let now = Instant::now();

    normalize_icu(norm, &src, &mut dest);

    let measured_us = now.elapsed().as_micros();

    let now = Instant::now();

    normalize_nfd_rust(&src, &mut dest);

    let measured_us2 = now.elapsed().as_micros();

    let now = Instant::now();

    let count = normalize_nfd_rust_count_only(&src);

    let measured_us3 = now.elapsed().as_micros();

    println!(
        "Normalization to NFD from {} of {} sample. ICU: {} us, Rust: {} us, Rust count {}: {} us",
        form, loc, measured_us, measured_us2, count, measured_us3
    );

    assert!(count < dest.len());

    // unsafe { unorm2_close_67(norm); }
}

fn measure_for_locale(loc: &str) {
    println!("");
    let sample = fs::read_to_string(format!("../../data/normalization/{}.txt", loc)).unwrap();

    let sample_nfc = sample.chars().nfc().collect::<String>();
    let sample_nfd = sample.chars().nfd().collect::<String>();

    measure_nfc(loc, &sample_nfc, "NFC");
    measure_nfd(loc, &sample_nfc, "NFC");
    measure_nfc(loc, &sample_nfd, "NFD");
    measure_nfd(loc, &sample_nfd, "NFD");

    if loc == "vi" {
        let sample_orthographic = sample_nfc
            .chars()
            .decompose_vietnamese_tones(true)
            .collect::<String>();
        measure_nfc(loc, &sample_orthographic, "orthographic");
        measure_nfd(loc, &sample_orthographic, "orthographic");
    }
}

fn main() {
    let locales = &["ar", "de", "en", "ko", "ru", "vi"];

    for loc in locales {
        measure_for_locale(loc);
    }
}

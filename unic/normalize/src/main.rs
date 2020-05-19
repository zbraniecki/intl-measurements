use normalize::ffi;

use std::fs;
use std::time::Instant;

use detone::IterDecomposeVietnamese;

use unicode_normalization::UnicodeNormalization;

fn measure_nfc(loc: &str, sample: &str, form: &str) {
    let mut err = 0;
    let norm = unsafe { ffi::unorm2_getNFCInstance_67(&mut err) };
    let src: Vec<u16> = sample.encode_utf16().collect();
    let mut dest_utf16: Vec<u16> = Vec::new();
    dest_utf16.resize(src.len() * 10, 0); // Assume the operation won't stretch the string by a factor larger than 10
    let mut dest_utf8 = String::with_capacity(src.len() * 10); // Tested with Vec<char>, no difference

    let now = Instant::now();

    normalize::icu(norm, &src, &mut dest_utf16);

    let measured_us = now.elapsed().as_micros();

    let now = Instant::now();

    normalize::nfc_rust_utf16(&src, &mut dest_utf16);

    let measured_us2 = now.elapsed().as_micros();

    let now = Instant::now();

    let count = normalize::nfc_rust_utf16_count_only(&src);

    let measured_us3 = now.elapsed().as_micros();

    println!(
        "Normalization to NFC using UTF-16 from {} of {} sample. ICU: {} us, Rust: {} us, Rust count {}: {} us",
        form, loc, measured_us, measured_us2, count, measured_us3
    );

    assert!(count < dest_utf16.len());

    let now = Instant::now();

    normalize::nfc_rust_utf8(sample, &mut dest_utf8);

    let measured_us = now.elapsed().as_micros();

    println!(
        "Normalization to NFC using UTF-8 from {} of {} sample. Rust: {} us",
        form, loc, measured_us
    );

    // unsafe { unorm2_close_67(norm); }
}

fn measure_nfd(loc: &str, sample: &str, form: &str) {
    let mut err = 0;
    let norm = unsafe { ffi::unorm2_getNFDInstance_67(&mut err) };
    let src: Vec<u16> = sample.encode_utf16().collect();
    let mut dest: Vec<u16> = Vec::new();
    dest.resize(src.len() * 10, 0); // Assume the operation won't stretch the string by a factor larger than 10
    let mut dest_utf8 = String::with_capacity(src.len() * 10); // Tested with Vec<char>, no difference

    let now = Instant::now();

    normalize::icu(norm, &src, &mut dest);

    let measured_us = now.elapsed().as_micros();

    let now = Instant::now();

    normalize::nfd_rust_utf16(&src, &mut dest);

    let measured_us2 = now.elapsed().as_micros();

    let now = Instant::now();

    let count = normalize::nfd_rust_utf16_count_only(&src);

    let measured_us3 = now.elapsed().as_micros();

    println!(
        "Normalization to NFD using UTF-16 from {} of {} sample. ICU: {} us, Rust: {} us, Rust count {}: {} us",
        form, loc, measured_us, measured_us2, count, measured_us3
    );

    assert!(count < dest.len());

    let now = Instant::now();

    normalize::nfc_rust_utf8(sample, &mut dest_utf8);

    let measured_us = now.elapsed().as_micros();

    println!(
        "Normalization to NFD using UTF-8 from {} of {} sample. Rust: {} us",
        form, loc, measured_us
    );

    // unsafe { unorm2_close_67(norm); }
}

fn measure_for_locale(loc: &str) {
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

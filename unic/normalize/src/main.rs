use std::time::Instant;
use std::fs;

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

fn measure_nfc(loc: &str, sample: String) {
    let mut err = 0;
    let norm = unsafe { unorm2_getNFCInstance_67(&mut err) };
    let src: Vec<u16> = sample.encode_utf16().collect();
    let length = src.len() as i32;
    let mut dest: Vec<u16> = Vec::with_capacity(length as usize);
    let capacity = dest.capacity() as i32;


    let now = Instant::now();

    let res = unsafe {
        unorm2_normalize_67(
            norm,
            src.as_ptr(),
            length,
            dest.as_mut_ptr(),
            capacity,
            &mut err);
    };

    let measured_us = now.elapsed().as_micros();

    let mut target = String::with_capacity(length as usize);
    let now = Instant::now();

    target.extend(sample.nfc());

    let measured_us2 = now.elapsed().as_micros();

    println!("Normalization to NFC of {} sample. ICU: {} us, Rust: {} us", loc, measured_us, measured_us2);

    // unsafe { unorm2_close_67(norm); }
}

fn measure_nfd(loc: &str, sample: String) {
    let mut err = 0;
    let norm = unsafe { unorm2_getNFDInstance_67(&mut err) };
    let src: Vec<u16> = sample.encode_utf16().collect();
    let length = src.len() as i32;
    let mut dest: Vec<u16> = Vec::with_capacity(length as usize);
    let capacity = dest.capacity() as i32;


    let now = Instant::now();

    let res = unsafe {
        unorm2_normalize_67(
            norm,
            src.as_ptr(),
            length,
            dest.as_mut_ptr(),
            capacity,
            &mut err);
    };

    let measured_us = now.elapsed().as_micros();

    let mut target = String::with_capacity(length as usize);
    let now = Instant::now();

    target.extend(sample.nfd());

    let measured_us2 = now.elapsed().as_micros();
    println!("Normalization to NFD of {} sample. ICU: {} us, Rust: {} us", loc, measured_us, measured_us2);

    // unsafe { unorm2_close_67(norm); }
}

fn measure_for_locale(loc: &str) {
    println!("");
    let sample = fs::read_to_string(format!("../../data/normalization/{}.txt", loc)).unwrap();

    // let sample_nfc;
    // let sample_nfd;

    measure_nfc(loc, sample.clone());
    measure_nfd(loc, sample.clone());
    // {
    //     let now = Instant::now();
    //
    //     let _ = Some(sample_nfc.nfd().collect::<String>());
    //
    //     let measured_us = now.elapsed().as_micros();
    //     println!("Normalization of NFC to NFD of {} sample. time: {} us", loc, measured_us);
    // }
    //
    // {
    //     let now = Instant::now();
    //
    //     let _ = Some(sample_nfd.nfc().collect::<String>());
    //
    //     let measured_us = now.elapsed().as_micros();
    //     println!("Normalization of NFD to NFC of {} sample. time: {} us", loc, measured_us);
    // }
}

fn main() {
    let locales = &["ar", "de", "en", "ko", "ru", "vi"];

    for loc in locales {
        measure_for_locale(loc);
    }
    // measure_c();
}

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use criterion::Throughput;
use detone::IterDecomposeVietnamese;
use std::fs;
use unicode_normalization::UnicodeNormalization;

fn measure_nfc(
    c: &mut Criterion,
    normalizer: *mut libc::c_void,
    loc: &str,
    sample: &str,
    form: &str,
) {
    let src: Vec<u16> = sample.encode_utf16().collect();
    let mut dest_utf16: Vec<u16> = Vec::new();
    dest_utf16.resize(src.len() * 10, 0); // Assume the operation won't stretch the string by a factor larger than 10
    let mut dest_utf8 = String::with_capacity(src.len() * 10); // Tested with Vec<char>, no difference

    let mut group = c.benchmark_group(format!("{}/nfc", loc));

    group.throughput(Throughput::Elements(sample.len() as u64));

    group.bench_with_input(format!("utf16/{}/icu", form), &src, |b, src| {
        b.iter(|| {
            normalize::icu(normalizer, &src, &mut dest_utf16);
        })
    });

    group.bench_with_input(format!("utf16/{}/rust", form), &src, |b, src| {
        b.iter(|| {
            normalize::nfc_rust_utf16(&src, &mut dest_utf16);
        })
    });

    group.bench_function(format!("utf16/{}/rust/count_only", form), |b| {
        b.iter(|| {
            let _ = normalize::nfc_rust_utf16_count_only(&src);
        })
    });

    group.bench_with_input(format!("utf8/{}/rust", form), &sample, |b, sample| {
        b.iter(|| {
            normalize::nfc_rust_utf8(sample, &mut dest_utf8);
        })
    });

    group.finish();
}

fn measure_nfd(
    c: &mut Criterion,
    normalizer: *mut libc::c_void,
    loc: &str,
    sample: &str,
    form: &str,
) {
    let src: Vec<u16> = sample.encode_utf16().collect();
    let mut dest_utf16: Vec<u16> = Vec::new();
    dest_utf16.resize(src.len() * 10, 0); // Assume the operation won't stretch the string by a factor larger than 10
    let mut dest_utf8 = String::with_capacity(src.len() * 10); // Tested with Vec<char>, no difference

    let mut group = c.benchmark_group(format!("{}/nfd", loc));

    group.throughput(Throughput::Elements(sample.len() as u64));

    group.bench_with_input(format!("utf16/{}/icu", form), &src, |b, src| {
        b.iter(|| {
            normalize::icu(normalizer, &src, &mut dest_utf16);
        })
    });

    group.bench_with_input(format!("utf16/{}/rust", form), &src, |b, src| {
        b.iter(|| {
            normalize::nfd_rust_utf16(&src, &mut dest_utf16);
        })
    });

    group.bench_function(format!("utf16/{}/rust/count_only", form), |b| {
        b.iter(|| {
            let _ = normalize::nfd_rust_utf16_count_only(&src);
        })
    });

    group.bench_with_input(format!("utf8/{}/rust", form), &sample, |b, sample| {
        b.iter(|| {
            normalize::nfd_rust_utf8(sample, &mut dest_utf8);
        })
    });

    group.finish();
}

fn measure(c: &mut Criterion) {
    let locales = &["ar", "de", "en", "ko", "ru", "vi"];

    let mut err = 0;
    let nfc_norm = unsafe { normalize::ffi::unorm2_getNFCInstance_67(&mut err) };
    let nfd_norm = unsafe { normalize::ffi::unorm2_getNFDInstance_67(&mut err) };

    for loc in locales {
        let sample = fs::read_to_string(format!("../../data/normalization/{}.txt", loc)).unwrap();
        let sample_nfc = sample.chars().nfc().collect::<String>();
        let sample_nfd = sample.chars().nfd().collect::<String>();

        measure_nfc(c, nfc_norm, loc, &sample_nfc, "nfc");
        measure_nfd(c, nfd_norm, loc, &sample_nfc, "nfc");
        measure_nfc(c, nfc_norm, loc, &sample_nfd, "nfd");
        measure_nfd(c, nfd_norm, loc, &sample_nfd, "nfd");

        if loc == &"vi" {
            let sample_orthographic = sample_nfc
                .chars()
                .decompose_vietnamese_tones(true)
                .collect::<String>();
            measure_nfc(c, nfc_norm, loc, &sample_orthographic, "orthographic");
            measure_nfd(c, nfd_norm, loc, &sample_orthographic, "orthographic");
        }
    }
}

criterion_group!(benches, measure);
criterion_main!(benches);

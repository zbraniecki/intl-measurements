use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use criterion::Throughput;
use std::fs;

fn measure_nfc(c: &mut Criterion) {
    let locales = &["en"];

    let mut err = 0;
    let nfc_norm = unsafe { normalize::ffi::unorm2_getNFCInstance_67(&mut err) };
    let nfd_norm = unsafe { normalize::ffi::unorm2_getNFDInstance_67(&mut err) };


    for loc in locales {
        let sample = fs::read_to_string(format!("../../data/normalization/{}.txt", loc)).unwrap();
        let src: Vec<u16> = sample.encode_utf16().collect();
        let mut dest_utf16: Vec<u16> = Vec::new();
        dest_utf16.resize(src.len() * 10, 0); // Assume the operation won't stretch the string by a factor larger than 10
        let mut dest_utf8 = String::with_capacity(src.len() * 10); // Tested with Vec<char>, no difference

        // NFC
        {
            let mut group = c.benchmark_group(format!("{}/nfc", loc));

            group.throughput(Throughput::Elements(sample.len() as u64));

            group.bench_with_input("utf16/icu", &src, |b, src| {
                b.iter(|| {
                    normalize::icu(nfc_norm, &src, &mut dest_utf16);
                })
            });

            group.bench_with_input("utf16/rust", &src, |b, src| {
                b.iter(|| {
                    normalize::nfc_rust_utf16(&src, &mut dest_utf16);
                })
            });

            group.bench_function("utf16/rust/count_only", |b| {
                b.iter(|| {
                    let _ = normalize::nfc_rust_utf16_count_only(&src);
                })
            });

            group.bench_with_input("utf8/rust", &sample, |b, sample| {
                b.iter(|| {
                    normalize::nfc_rust_utf8(sample, &mut dest_utf8);
                })
            });

            group.finish();
        }

        // NFD
        {
            let mut group = c.benchmark_group(format!("{}/nfd", loc));

            group.throughput(Throughput::Elements(sample.len() as u64));

            group.bench_with_input("utf16/icu", &src, |b, src| {
                b.iter(|| {
                    normalize::icu(nfd_norm, &src, &mut dest_utf16);
                })
            });

            group.bench_with_input("utf16/rust", &src, |b, src| {
                b.iter(|| {
                    normalize::nfd_rust_utf16(&src, &mut dest_utf16);
                })
            });

            group.bench_function("utf16/rust/count_only", |b| {
                b.iter(|| {
                    let _ = normalize::nfd_rust_utf16_count_only(&src);
                })
            });

            group.bench_with_input("utf8/rust", &sample, |b, sample| {
                b.iter(|| {
                    normalize::nfd_rust_utf8(sample, &mut dest_utf8);
                })
            });

            group.finish();
        }
    }
}


criterion_group!(benches, measure_nfc);
criterion_main!(benches);

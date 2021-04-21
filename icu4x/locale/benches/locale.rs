use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use icu_locid::Locale;
use intl_harness::data::locale::get_test_set;

fn locale_parsing(c: &mut Criterion) {
    let data_set = get_test_set("../../harness/data");
    let strings: Vec<_> = data_set.0.iter().map(|test| test.input.clone()).collect();

    c.bench_function("locale_parsing", |b| {
        b.iter(|| {
            for s in &strings {
                let _: Result<Locale, _> = black_box(s).parse();
            }
        })
    });
}

criterion_group!(benches, locale_parsing,);
criterion_main!(benches);

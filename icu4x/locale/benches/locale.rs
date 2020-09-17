use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use icu_locale::Locale;
use locale::STRINGS;

fn locale_parsing(c: &mut Criterion) {
    c.bench_function("locale_parsing", |b| {
        b.iter(|| {
            for s in STRINGS {
                let _: Result<Locale, _> = black_box(s).parse();
            }
        })
    });
}

criterion_group!(benches, locale_parsing,);
criterion_main!(benches);

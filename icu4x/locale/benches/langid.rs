use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use icu_locale::LanguageIdentifier;
use locale::STRINGS;

fn language_identifier(c: &mut Criterion) {
    c.bench_function("language_identifier_parsing", |b| {
        b.iter(|| {
            for s in STRINGS {
                let _: Result<LanguageIdentifier, _> = black_box(s).parse();
            }
        })
    });

    c.bench_function("language_identifier_matches", |b| {
        let locales: Vec<LanguageIdentifier> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();
        let reference_locale: LanguageIdentifier = "en-US".parse().unwrap();
        let mut matches = 0;

        b.iter(|| {
            for loc in &locales {
                if loc == &reference_locale {
                  matches += 1;
                }
            }
        })
    });

    c.bench_function("language_identifier_serialize", |b| {
        let locales: Vec<LanguageIdentifier> = STRINGS.iter().map(|s| s.parse().unwrap()).collect();

        b.iter(|| {
            for loc in &locales {
                let _ = black_box(loc).to_string();
            }
        })
    });
}


criterion_group!(benches, language_identifier);
criterion_main!(benches);

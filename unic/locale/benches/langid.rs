use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use unic_langid::LanguageIdentifier;
use locale::STRINGS;

fn language_identifier_parsing(c: &mut Criterion) {
    c.bench_function("language_identifier_parsing", |b| {
        b.iter(|| {
            for s in STRINGS {
                let _: Result<LanguageIdentifier, _> = black_box(s).parse();
            }
        })
    });
}

fn language_identifier_matches(c: &mut Criterion) {
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
}


criterion_group!(benches, language_identifier_parsing, language_identifier_matches);
criterion_main!(benches);

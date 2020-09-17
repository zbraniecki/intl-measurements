use icu_locid::LanguageIdentifier;
use intl_harness::locale::HarnessLocaleRuntime;

pub struct Icu4XLocale;

impl HarnessLocaleRuntime for Icu4XLocale {
    type Locale = LanguageIdentifier;

    fn canonicalize(input: &str) -> String {
        LanguageIdentifier::canonicalize(input).unwrap()
    }
}

fn main() {
    Icu4XLocale::run("../../harness/data");
}

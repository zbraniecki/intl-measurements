use unic_langid::LanguageIdentifier;
use unic_langid::canonicalize;
use intl_harness::locale::HarnessLocaleRuntime;

pub struct UnicLocale;

impl HarnessLocaleRuntime for UnicLocale {
    type Locale = LanguageIdentifier;

    fn canonicalize(input: &str) -> String {
        canonicalize(input).unwrap()
    }
}

fn main() {
    UnicLocale::run("../../harness/data");
}

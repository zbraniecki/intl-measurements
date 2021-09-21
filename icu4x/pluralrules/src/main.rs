use std::convert::TryFrom;
use icu_plurals::{PluralRules, PluralRuleType, PluralOperands};
use icu_locid::LanguageIdentifier;
use icu_provider_blob::StaticDataProvider;
use icu_provider_fs::FsDataProvider;
use intl_harness::plurals::HarnessPluralsRuntime;

pub struct Icu4XPluralRules {
    provider: StaticDataProvider,
    langids: Vec<LanguageIdentifier>,
}

const CLDR_BLOB: &[u8] = include_bytes!(concat!(
    "../../data/icu4x/cldr39.postcard"
));

impl Icu4XPluralRules {
    fn new() -> Self {
        let provider = StaticDataProvider::new_from_static_blob(&CLDR_BLOB)
            .expect("Failed to load data");
        Self {
            provider,
            langids: vec![]
        }
    }
}
 
impl HarnessPluralsRuntime for Icu4XPluralRules {
    fn prepare(&mut self, langids: &[String]) {
        let langids: Vec<LanguageIdentifier> = langids.iter().map(|l| l.parse().unwrap()).collect();
        self.langids.extend(langids);
    }

    fn select(&self, values: &[isize]) -> Vec<String> {
        for langid in &self.langids {
            let pr = PluralRules::try_new(langid.clone(), &self.provider, PluralRuleType::Cardinal).unwrap();
            for value in values {
                let operands = PluralOperands::try_from(*value).unwrap();
                let _ = pr.select(operands);
            }
        }
        vec![]
    }
}
 
fn main() {
    let mut runner = Icu4XPluralRules::new();
    runner.run("../../harness/data");
}

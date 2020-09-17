use intl_harness::plurals::HarnessPluralsRuntime;
use intl_pluralrules::{PluralRules, PluralRuleType};
use unic_langid::LanguageIdentifier;

pub struct UnicPluralRules {
    langids: Vec<LanguageIdentifier>,
}

impl UnicPluralRules {
    fn new() -> Self {
        Self {
            langids: vec![]
        }
    }
}
 
impl HarnessPluralsRuntime for UnicPluralRules {
    fn prepare(&mut self, langids: &[String]) {
        let langids: Vec<LanguageIdentifier> = langids.iter().map(|l| l.parse().unwrap()).collect();
        self.langids.extend(langids);
    }

    fn select(&self, values: &[isize]) -> Vec<String> {
        for langid in &self.langids {
            let pr = PluralRules::create(langid.clone(), PluralRuleType::CARDINAL).unwrap();
            for value in values {
                let _ = pr.select(*value);
            }
        }
        vec![]
    }
}
 
fn main() {
    let mut runner = UnicPluralRules::new();
    runner.run("../../harness/data");
}

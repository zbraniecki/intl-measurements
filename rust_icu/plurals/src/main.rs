use rust_icu_upluralrules::UPluralRules;
use intl_harness::plurals::HarnessPluralsRuntime;

pub struct RustIcuPluralRules {
    langids: Vec<String>,
}

impl RustIcuPluralRules {
    fn new() -> Self {
        Self {
            langids: vec![]
        }
    }
}
 
impl HarnessPluralsRuntime for RustIcuPluralRules {
    fn prepare(&mut self, langids: &[String]) {
        self.langids.extend(langids.iter().cloned().collect::<Vec<_>>());
    }

    fn select(&self, values: &[isize]) -> Vec<String> {
        for langid in &self.langids {
            let pr = UPluralRules::try_new(langid).unwrap();
            for value in values {
                let _ = pr.select(*value as f64);
            }
        }
        vec![]
    }
}
 
fn main() {
    let mut runner = RustIcuPluralRules::new();
    runner.run("../../harness/data");
}

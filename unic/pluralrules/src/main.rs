use std::time::Instant;
use intl_pluralrules::{PluralRules, PluralRuleType};
use unic_langid::{LanguageIdentifier, langid};

const LOCALES: &[LanguageIdentifier] = &[
    langid!("uk"),
    langid!("de"),
    langid!("sk"),
    langid!("ar"),
    langid!("fr"),
    langid!("it"),
    langid!("en"),
    langid!("cs"),
    langid!("es"),
    langid!("zh")
];
const SAMPLES: &[isize] = &[
    1,
    2,
    3,
    4,
    5,
    25,
    134,
    910293019,
    12,
    1412,
    -12,
    15,
    2931,
    31231,
    3123,
    13231,
    91,
    0,
    231,
    -2,
    -45,
    33,
    728,
    2,
    291,
    24,
    479,
    291,
    778,
    919,
    93
];

fn main() {
    let now = Instant::now();

    for loc in LOCALES {
        let pr = PluralRules::create(loc.clone(), PluralRuleType::CARDINAL).unwrap();
        for sample in SAMPLES {
            let _ = pr.select(*sample);
        }
    }

    println!("Select {} numbers for {} locales: {} ns", SAMPLES.len(), LOCALES.len(), now.elapsed().as_nanos());
}

use rust_icu_uloc::ULoc;
use rust_icu_common::Error as RustIcuError;
use intl_harness::locale::HarnessLocaleRuntime;
use std::str::FromStr;
use std::ops::Deref;
use std::fmt::Display;

pub struct RustIcuLocale;

#[derive(PartialEq)]
pub struct LanguageIdentifier(ULoc);

impl Deref for LanguageIdentifier {
    type Target = ULoc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for LanguageIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for LanguageIdentifier {
    type Err = RustIcuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(ULoc::for_language_tag(s)?))
    }
}

impl HarnessLocaleRuntime for RustIcuLocale {
    type Locale = LanguageIdentifier;

    fn canonicalize(input: &str) -> String {
        let loc: LanguageIdentifier = input.parse().unwrap();
        loc.canonicalize().unwrap().to_string()
    }
}

fn main() {
    RustIcuLocale::run("../../harness/data");
}

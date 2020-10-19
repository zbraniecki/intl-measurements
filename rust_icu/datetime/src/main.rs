use chrono::naive::NaiveDateTime;
use intl_harness::datetime::HarnessDateTimeRuntime;
use rust_icu_sys::UDate;
use rust_icu_sys::UDateFormatStyle;
use rust_icu_udat::UDateFormat;
use rust_icu_uloc::ULoc;
use rust_icu_ustring::UChar;
use std::convert::TryFrom;

fn parse_datetime(input: &str) -> UDate {
    let dt = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S:%3f").unwrap();
    dt.timestamp() as UDate
}

pub struct RustIcuDateTime {
    langids: Vec<ULoc>,
    dates: Vec<UDate>,
    styles: Vec<(String, String)>,
}

impl RustIcuDateTime {
    fn new() -> Self {
        Self {
            langids: vec![],
            dates: vec![],
            styles: vec![],
        }
    }
}

fn get_style(input: &str) -> UDateFormatStyle {
    match input {
        "none" => UDateFormatStyle::UDAT_NONE,
        "short" => UDateFormatStyle::UDAT_SHORT,
        "medium" => UDateFormatStyle::UDAT_MEDIUM,
        "long" => UDateFormatStyle::UDAT_LONG,
        "full" => UDateFormatStyle::UDAT_FULL,
        _ => panic!(),
    }
}

impl HarnessDateTimeRuntime for RustIcuDateTime {
    fn prepare(&mut self, langids: &[String], values: &[String], styles: &[(String, String)]) {
        self.langids = langids
            .iter()
            .map(|l| ULoc::for_language_tag(l).unwrap())
            .collect();
        self.dates = values.iter().map(|v| parse_datetime(v)).collect();
        self.styles = styles.iter().cloned().collect();
    }

    fn format(&self) -> Vec<String> {
        let tz_id = UChar::try_from("America/New_York").unwrap();
        for langid in &self.langids {
            for style in &self.styles {
                let dtf = UDateFormat::new_with_styles(
                    get_style(&style.0),
                    get_style(&style.1),
                    langid,
                    &tz_id,
                )
                .unwrap();
                for value in &self.dates {
                    let _ = dtf.format(*value);
                }
            }
        }
        vec![]
    }
}

fn main() {
    let mut runner = RustIcuDateTime::new();
    runner.run("../../harness/data");
}

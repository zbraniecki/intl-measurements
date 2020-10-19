use icu_datetime::DateTimeFormat;
use icu_datetime::date::MockDateTime;
use icu_datetime::options::{self, style};
use icu_provider_fs::FsDataProvider;
use icu_locid::LanguageIdentifier;
use intl_harness::datetime::HarnessDateTimeRuntime;


pub struct Icu4XDateTime {
    provider: FsDataProvider,
    langids: Vec<LanguageIdentifier>,
    dates: Vec<MockDateTime>,
    styles: Vec<(String, String)>,
}

impl Icu4XDateTime {
    fn new() -> Self {
        let provider = FsDataProvider::try_new("../data/icu4x/bincode")
            .expect("Loading file from testdata directory");
        Self {
            provider,
            langids: vec![],
            dates: vec![],
            styles: vec![],
        }
    }
}

fn get_date_style(input: &str) -> Option<style::Date> {
    match input {
        "none" => None,
        "short" => Some(style::Date::Short),
        "medium" => Some(style::Date::Medium),
        "long" => Some(style::Date::Long),
        "full" => Some(style::Date::Full),
        _ => panic!()
    }
}

fn get_time_style(input: &str) -> Option<style::Time> {
    match input {
        "none" => None,
        "short" => Some(style::Time::Short),
        "medium" => Some(style::Time::Medium),
        "long" => Some(style::Time::Long),
        "full" => Some(style::Time::Full),
        _ => panic!()
    }
}

impl HarnessDateTimeRuntime for Icu4XDateTime {
    fn prepare(&mut self, langids: &[String], values: &[String], styles: &[(String, String)]) {
        let langids: Vec<LanguageIdentifier> = langids.iter().map(|l| l.parse().unwrap()).collect();
        self.langids.extend(langids);
        self.dates = values.iter().map(|s| s.parse().unwrap()).collect();
        self.styles = styles.iter().cloned().collect();
    }

    fn format(&self) -> Vec<String> {
        for langid in &self.langids {
            for style in &self.styles {
                let options = style::Bag {
                    date: get_date_style(&style.0),
                    time: get_time_style(&style.1),
                    preferences: None,
                }.into();
                let dtf = DateTimeFormat::try_new(langid.clone(), &self.provider, &options).unwrap();
                for value in &self.dates {
                    let _ = dtf.format(value);
                }
            }
        }
        vec![]
    }
}
 
fn main() {
    let mut runner = Icu4XDateTime::new();
    runner.run("../../harness/data");
}

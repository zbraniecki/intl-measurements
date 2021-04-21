use icu_datetime::mock::datetime::MockDateTime;
use icu_datetime::{options::length, DateTimeFormat};
use icu_provider_fs::FsDataProvider;
use icu_locid::LanguageIdentifier;
use intl_harness::datetime::HarnessDateTimeRuntime;


pub struct Icu4XDateTime {
    provider: FsDataProvider,
    langids: Vec<LanguageIdentifier>,
    dates: Vec<MockDateTime>,
    lengths: Vec<(String, String)>,
}

impl Icu4XDateTime {
    fn new() -> Self {
        let provider = FsDataProvider::try_new("../data/icu4x/bincode")
            .expect("Loading file from testdata directory");
        Self {
            provider,
            langids: vec![],
            dates: vec![],
            lengths: vec![],
        }
    }
}

fn get_date_length(input: &str) -> Option<length::Date> {
    match input {
        "none" => None,
        "short" => Some(length::Date::Short),
        "medium" => Some(length::Date::Medium),
        "long" => Some(length::Date::Long),
        "full" => Some(length::Date::Full),
        _ => panic!()
    }
}

fn get_time_length(input: &str) -> Option<length::Time> {
    match input {
        "none" => None,
        "short" => Some(length::Time::Short),
        "medium" => Some(length::Time::Medium),
        "long" => Some(length::Time::Long),
        "full" => Some(length::Time::Full),
        _ => panic!()
    }
}

impl HarnessDateTimeRuntime for Icu4XDateTime {
    fn prepare(&mut self, langids: &[String], values: &[String], lengths: &[(String, String)]) {
        let langids: Vec<LanguageIdentifier> = langids.iter().map(|l| l.parse().unwrap()).collect();
        self.langids.extend(langids);
        self.dates = values.iter().map(|s| s.parse().unwrap()).collect();
        self.lengths = lengths.iter().cloned().collect();
    }

    fn format(&self) -> Vec<String> {
        for langid in &self.langids {
            for length in &self.lengths {
                let options = length::Bag {
                    date: get_date_length(&length.0),
                    time: get_time_length(&length.1),
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

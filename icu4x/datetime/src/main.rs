use std::time::Instant;
use icu_datetime::DateTimeFormat;
use icu_datetime::date::MockDateTime;
use icu_datetime::options::{self, style};
use icu_provider_fs::FsDataProvider;
use icu_locid::LanguageIdentifier;

fn main() {
    let dates = &[
        "2001-09-08T18:46:40:000",
        "2017-07-13T19:40:00:000",
        "2020-09-13T05:26:40:000",
        "2021-01-06T22:13:20:000",
        "2021-05-02T17:00:00:000",
        "2021-08-26T10:46:40:000",
        "2021-11-20T03:33:20:000",
        "2022-04-14T22:20:00:000",
        "2022-08-08T16:06:40:000",
        "2033-05-17T20:33:20:000"
    ];
    let dates: Vec<MockDateTime> = dates.into_iter().map(|d| d.parse().unwrap()).collect();
    let values = &[
        ("pl", Some(style::Date::Full), None),
        ("pl", Some(style::Date::Long), None),
        ("pl", Some(style::Date::Medium), None),
        ("pl", Some(style::Date::Short), None),
        // ("pl", None, Some(style::Time::Full)),
        // ("pl", None, Some(style::Time::Long)),
        ("pl", None, Some(style::Time::Medium)),
        ("pl", None, Some(style::Time::Short)),
        ("pl", Some(style::Date::Full), Some(style::Time::Medium)),
        ("pl", Some(style::Date::Long), Some(style::Time::Medium)),
        ("pl", Some(style::Date::Medium), Some(style::Time::Medium)),
        ("pl", Some(style::Date::Short), Some(style::Time::Short)),
    ];

    println!("Measuring formatting date/time");


    let now = Instant::now();


    let provider = FsDataProvider::try_new("../data/icu4x/bincode")
        .expect("Loading file from testdata directory");

    let mut s = String::new();

    for value in values {
        let langid: LanguageIdentifier = value.0.parse().unwrap();

        let dtf = DateTimeFormat::try_new(langid, &provider, &options::DateTimeFormatOptions::Style(
            options::style::Bag {
                date: value.1,
                time: value.2,
                ..Default::default()
            }
        )).unwrap();
        for date in &dates {
            dtf.format_to_write(&mut s, date).unwrap();
            // println!("{:#?}", s);
            s.clear();
        }
    }

    let elapsed = now.elapsed();
    println!("us: {:#?}", elapsed.as_micros());
}

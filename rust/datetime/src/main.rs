use std::env;
use std::time::Instant;
use unic_datetime::data::load_json; // CLDR loading
use unic_datetime::data::load_bin; // bincode loading
use unic_datetime::*;

#[derive(Debug)]
enum Modes {
    JSON,
    Binary,
    Inline,
}

impl Modes {
    pub fn from(arg: Option<&String>) -> Modes {
        match arg.map(|a| a.as_str()) {
            Some("json") => Modes::JSON,
            Some("bin") => Modes::Binary,
            Some("inline") => Modes::Inline,
            None => Modes::Inline,
            _ => panic!("Unknown mode!"),
        }
    }
}

fn main() {
    let dates = &[
        DateTime::new(2001, 9, 8, 18, 46, 40),
        DateTime::new(2017, 7, 13, 19, 40, 0),
        DateTime::new(2020, 9, 13, 5, 26, 40),
        DateTime::new(2021, 1, 6, 22, 13, 20),
        DateTime::new(2021, 5, 2, 17, 0, 0),
        DateTime::new(2021, 8, 26, 10, 46, 40),
        DateTime::new(2021, 12, 20, 3, 33, 20),
        DateTime::new(2022, 4, 14, 22, 20, 0),
        DateTime::new(2022, 8, 8, 16, 6, 40),
        DateTime::new(2033, 5, 17, 20, 33, 20),
    ];
    let values = &[
        ("pl", Some(DateStyle::FULL), None),
        ("pl", Some(DateStyle::LONG), None),
        ("pl", Some(DateStyle::MEDIUM), None),
        ("pl", Some(DateStyle::SHORT), None),
        // ("pl", None, Some(TimeStyle::FULL)),
        // ("pl", None, Some(TimeStyle::LONG)),
        ("pl", None, Some(TimeStyle::MEDIUM)),
        ("pl", None, Some(TimeStyle::SHORT)),
        ("pl", Some(DateStyle::FULL), Some(TimeStyle::MEDIUM)),
        ("pl", Some(DateStyle::LONG), Some(TimeStyle::MEDIUM)),
        ("pl", Some(DateStyle::MEDIUM), Some(TimeStyle::MEDIUM)),
        ("pl", Some(DateStyle::SHORT), Some(TimeStyle::SHORT)),
    ];

    let args: Vec<String> = env::args().collect();
    let mode = Modes::from(args.get(1));
    let data = match mode {
        Modes::JSON => load_json::get_calendar_data("./data/cldr-dates-modern", "pl"),
        Modes::Binary => load_bin::get_calendar_data("./res", "pl"),
        Modes::Inline => data::pl::RESOURCE,
    };
    // println!("{:#?}", data);
    println!("Measuring formatting date/time in mode: {:#?}", mode);

    let now = Instant::now();


    for value in values {
        let dtf = DateTimeFormat::new(value.0, value.1, value.2, &data);
        for date in dates {
            let _ = dtf.format(date);
            // println!("{}", s);
        }
    }

    let elapsed = now.elapsed();
    println!("us: {:#?}", elapsed.as_micros());
}

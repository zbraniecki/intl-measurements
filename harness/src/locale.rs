use crate::data::locale;
use crate::structs::*;
use std::fmt::Debug;
use std::mem;
use std::str::FromStr;
use std::time::Instant;

pub trait HarnessLocaleRuntime {
    type Locale;

    fn canonicalize(input: &str) -> String;

    fn run(data_path: &str)
    where
        Self::Locale: PartialEq + ToString + FromStr,
        <Self::Locale as FromStr>::Err: Debug,
    {
        let mut suit_result = TestSuitResult::default();

        let strings = locale::get_test_set(data_path);

        let inputs: Vec<&str> = strings.0.iter().map(|set| set.input.as_str()).collect();

        let parsed = {
            let test = Test {
                id: "langid/construct".to_string(),
                description: format!("Create LanguageIdentifier from str for {} locales", inputs.len()),
                unit: "ns".into(),
            };

            let now = Instant::now();

            let output: Vec<Self::Locale> = inputs
                .iter()
                .map(|s| s.parse().unwrap())
                .collect();

            let measured_ns = now.elapsed().as_nanos();

            let result = TestResult {
                value: measured_ns,
                unit: "ns".into(),
            };

            suit_result.tests.push((test, result));
            
            output
        };

        {
            let reference_str = "en-US";
            let test = Test {
                id: "langid/filter".to_string(),
                description: format!("Filter out locales that match {}", reference_str),
                unit: TestUnit::Time(TestUnitTime::Nano)
            };
            let reference: Self::Locale = reference_str.parse().unwrap();
            let now = Instant::now();
            let _: Vec<_> = parsed.iter().filter(|loc| *loc == &reference).collect();
            let measured_ns = now.elapsed().as_nanos();
            let result = TestResult {
                value: measured_ns,
                unit: "ns".into(),
            };
            suit_result.tests.push((test, result));
        }

        {
            let test = Test {
                id: "langid/serialize".to_string(),
                description: format!("Serialize {} locales", parsed.len()),
                unit: TestUnit::Time(TestUnitTime::Nano)
            };
            let now = Instant::now();
            let _: Vec<_> = parsed.iter().map(|lid| lid.to_string()).collect();
            let measured_ns = now.elapsed().as_nanos();
            let result = TestResult {
                value: measured_ns,
                unit: "ns".into(),
            };
            suit_result.tests.push((test, result));
        };

        {
            let test = Test {
                id: "langid/canonicalize".to_string(),
                description: format!("Canonicalize {} locales", inputs.len()),
                unit: TestUnit::Time(TestUnitTime::Nano)
            };
            let now = Instant::now();
            let _: Vec<_> = inputs.iter().map(|i| Self::canonicalize(i)).collect();
            let measured_ns = now.elapsed().as_nanos();
            let result = TestResult {
                value: measured_ns,
                unit: "ns".into(),
            };
            suit_result.tests.push((test, result));
        };

        {
            // Size

            // XXX: This does not measure heap allocation, leaving `variants` not accounted for.
            // Since there are only 3 locales with variants in the sample, we hope the results
            // remain meaningful, but it would be nice to improve the measuring.
            let test = Test {
                id: "langid/memory".to_string(),
                description: format!("Total size of locales vector for {} locales", parsed.len()),
                unit: TestUnit::Size(TestUnitSize::Bytes)
            };
            let size = mem::size_of::<Self::Locale>() * parsed.capacity();
            let result = TestResult {
                value: size as u128,
                unit: "b".into(),
            };
            suit_result.tests.push((test, result));
        }

        suit_result.print();
    }
}

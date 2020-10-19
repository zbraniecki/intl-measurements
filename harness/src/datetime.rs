use crate::data::datetime;
use crate::structs::*;
use std::time::Instant;

pub trait HarnessDateTimeRuntime {
    fn prepare(&mut self, langids: &[String], values: &[String], styles: &[(String, String)]);

    fn format(&self) -> Vec<String>;

    fn run(&mut self, data_path: &str) {
        let mut suit_result = TestSuitResult::default();

        let set = datetime::get_test_set(data_path);
        let styles: Vec<(String, String)> = set.styles.iter().map(|s| (s.date.clone(), s.time.clone())).collect();

        self.prepare(&set.langids, &set.values, &styles);

        {
            let test = Test {
                id: "datetime/format".to_string(),
                description: format!("Format Date & Time in 10 locales and 10 styles"),
                unit: "ns".into(),
            };
            let now = Instant::now();

            let _: Vec<_> = self.format();
            let measured_ns = now.elapsed().as_micros();
            let result = TestResult {
                value: measured_ns,
                unit: "ns".into(),
            };
            suit_result.tests.push((test, result));
            result
        };

        suit_result.print();
    }
}


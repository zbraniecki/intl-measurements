use crate::data::plurals;
use crate::structs::*;
use std::time::Instant;

pub trait HarnessPluralsRuntime {
    fn prepare(&mut self, langids: &[String]);

    fn select(&self, values: &[isize]) -> Vec<String>;

    fn run(&mut self, data_path: &str) {
        let mut suit_result = TestSuitResult::default();

        let set = plurals::get_test_set(data_path);

        self.prepare(&set.langids);

        {
            let test = Test {
                id: "plurals/select".to_string(),
                description: format!("Select category for 10 numbers in 10 locales"),
                unit: "ns".into(),
            };
            let now = Instant::now();
            let _: Vec<_> = self.select(&set.values.isize);
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


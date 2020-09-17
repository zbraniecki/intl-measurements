use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestSet {
    pub langids: Vec<String>,
    pub values: Values,
}

#[derive(Serialize, Deserialize)]
pub struct Values {
    pub isize: Vec<isize>,
}

pub fn get_test_set(data_path: &str) -> TestSet {
    crate::get_fixture(&format!("{}/pluralrules/plurals.json", data_path)).unwrap()
}

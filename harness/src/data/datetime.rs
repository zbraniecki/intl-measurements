use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestSet {
    pub langids: Vec<String>,
    pub values: Vec<String>,
    pub styles: Vec<Style>,
}

#[derive(Serialize, Deserialize)]
pub struct Style {
    pub date: String,
    pub time: String,
}

pub fn get_test_set(data_path: &str) -> TestSet {
    crate::get_fixture(&format!("{}/datetime/datetime.json", data_path)).unwrap()
}

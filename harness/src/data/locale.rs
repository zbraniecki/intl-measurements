use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestSet(pub Vec<Test>);

#[derive(Serialize, Deserialize)]
pub struct Test {
    pub input: String,
    pub output: String,
}

pub fn get_test_set(data_path: &str) -> TestSet {
    crate::get_fixture(&format!("{}/locale/langids.json", data_path)).unwrap()
}

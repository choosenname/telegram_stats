use serde::de::DeserializeOwned;
use std::fs;
use std::fs::File;
use std::io::BufReader;

pub struct DataGetter;

impl DataGetter {
    pub async fn process_file_from_str<T: DeserializeOwned>(
        &self,
        file_path: &str,
    ) -> anyhow::Result<T> {
        let file_content = fs::read_to_string(file_path)?;
        serde_json::from_str(&file_content).map_err(Into::into)
    }

    pub async fn process_file_from_reader<T: DeserializeOwned>(
        file_path: &str,
    ) -> anyhow::Result<T> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(Into::into)
    }
}

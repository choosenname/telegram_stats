use serde::de::DeserializeOwned;
use std::fs;
use std::fs::File;
use std::io::BufReader;

pub struct DataGetter;

type Result<T> = core::result::Result<T, DataGetterError>;

impl DataGetter {
    #[allow(dead_code)]
    pub async fn process_file_from_str<T: DeserializeOwned>(
        file_path: &str,
    ) -> Result<T> {
        let file_content = fs::read_to_string(file_path).map_err(DataGetterError::ReadFile)?;
        serde_json::from_str(&file_content).map_err(DataGetterError::ParseData)
    }

    pub async fn process_file_from_reader<T: DeserializeOwned>(
        file_path: &str,
    ) -> Result<T> {
        let file = File::open(file_path).map_err(DataGetterError::ReadFile)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(DataGetterError::ParseData)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DataGetterError{
    #[error("Failed to read file: 0")]
    ReadFile(#[source] std::io::Error),
    #[error("Failed to parse data: 0")]
    ParseData(#[source] serde_json::Error),
}

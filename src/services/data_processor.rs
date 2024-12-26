use std::error::Error;
use std::fs;
use crate::core::types::chat::Chat;
use crate::data::repositories::data_repository::DataRepository;

pub struct DataProcessor<D: DataRepository> {
    pub data_repository: D,
}

impl<D: DataRepository> DataProcessor<D> {
    pub fn new(data_repository: D) -> Self {
        Self {
            data_repository,
        }
    }

    pub async fn process_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file_content = fs::read_to_string(file_path)?;
        let data: Chat = serde_json::from_str(&file_content)?;
        dbg!(data);
        todo!()
    }
}

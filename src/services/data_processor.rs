use crate::core::types::chat::Chat;
use crate::data::repositories::data_repository::DataRepository;
use std::error::Error;
use std::fs;

pub struct DataProcessor<D: DataRepository<Data = Chat>> {
    pub data_repository: D,
}

impl<D: DataRepository<Data = Chat>> DataProcessor<D> {
    pub fn new(data_repository: D) -> Self {
        Self { data_repository }
    }

    pub async fn process_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file_content = fs::read_to_string(file_path)?;
        let data: Chat = serde_json::from_str(&file_content)?;
        // dbg!(data);
        self.data_repository.save(data).await?;
        Ok(())
    }
}

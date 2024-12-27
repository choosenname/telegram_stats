use std::error::Error;
use crate::data::repositories::data_repository::DataRepository;
use serde::de::DeserializeOwned;
use std::fs;
use std::fs::File;
use std::io::BufReader;

pub struct DataProcessor<D: DataRepository> {
    pub data_repository: D,
}

impl<D> DataProcessor<D>
where
    D: DataRepository<Data: DeserializeOwned>,
{
    pub fn new(data_repository: D) -> Self {
        Self { data_repository }
    }

    pub async fn process_file_from_str(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file_content = fs::read_to_string(file_path)?;
        let data: D::Data = serde_json::from_str(&file_content)?;
        self.data_repository.save(data).await?;
        Ok(())
    }

    pub async fn process_file_from_reader(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let data: D::Data = serde_json::from_reader(reader)?;
        self.data_repository.save(data).await?;
        Ok(())
    }
}

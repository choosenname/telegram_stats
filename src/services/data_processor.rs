use crate::data::models::data_getter::DataGetter;
use crate::data::repositories::data_repository::DataRepository;
use serde::de::DeserializeOwned;

pub struct DataProcessor<D> {
    pub data_repository: D,
}

impl<D> DataProcessor<D>
where
    D: DataRepository<Data: DeserializeOwned>,
{
    pub fn new(data_repository: D) -> Self {
        Self { data_repository }
    }

    pub async fn process_file_and_save(&self, file_path: &str) -> anyhow::Result<()> {
        let data: D::Data = DataGetter::process_file_from_reader(file_path).await?;
        self.data_repository.save(data).await?;
        Ok(())
    }
}

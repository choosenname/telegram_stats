pub trait DataRepository {
    type Data;

    async fn save(&self, data: Self::Data) -> Result<(), DataError>;
}

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("save error")]
    Save,
}

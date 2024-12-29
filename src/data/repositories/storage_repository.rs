pub trait StorageRepository {
    type Data;

    async fn save(&self, data: Self::Data) -> Result<(), StorageError>;
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("save error")]
    Save,
}

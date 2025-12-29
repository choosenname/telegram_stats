#[allow(dead_code)]
pub trait StorageRepository {
    type Data;

    async fn save(&self, data: Self::Data) -> Result<(), StorageError>;
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum StorageError {
    #[error("save error")]
    Save,
}

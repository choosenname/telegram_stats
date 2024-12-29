use serde::Serialize;

pub trait DataRepository {
    async fn save<T: Serialize>(&self, data: &T) -> Result<()>;
}

pub type Result<T> = std::result::Result<T, DataError>;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("save error")]
    Save,
}

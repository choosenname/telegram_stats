#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("save error")]
    Save,
}
